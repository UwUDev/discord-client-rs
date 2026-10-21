use crate::{BoxedError, BoxedResult};
use serde_json::{Map, Number, Value};

const VERSION: u8 = 131;
const MAX_SAFE_INTEGER: u64 = (1 << 53) - 1;

pub fn encode(value: &Value) -> Vec<u8> {
    let mut out = vec![VERSION];
    write_value(&mut out, value);
    out
}

fn write_value(out: &mut Vec<u8>, value: &Value) {
    match value {
        Value::Null => write_atom(out, "nil"),
        Value::Bool(true) => write_atom(out, "true"),
        Value::Bool(false) => write_atom(out, "false"),
        Value::Number(n) => write_number(out, n),
        Value::String(s) => write_binary(out, s.as_bytes()),
        Value::Array(items) => {
            if items.is_empty() {
                out.push(106);
                return;
            }
            out.push(108);
            out.extend_from_slice(&(items.len() as u32).to_be_bytes());
            for item in items {
                write_value(out, item);
            }
            out.push(106);
        }
        Value::Object(map) => {
            out.push(116);
            out.extend_from_slice(&(map.len() as u32).to_be_bytes());
            for (k, v) in map {
                write_binary(out, k.as_bytes());
                write_value(out, v);
            }
        }
    }
}

fn write_atom(out: &mut Vec<u8>, atom: &str) {
    out.push(115);
    out.push(atom.len() as u8);
    out.extend_from_slice(atom.as_bytes());
}

fn write_binary(out: &mut Vec<u8>, bytes: &[u8]) {
    out.push(109);
    out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
    out.extend_from_slice(bytes);
}

fn write_number(out: &mut Vec<u8>, n: &Number) {
    if let Some(f) = n.as_f64().filter(|_| !n.is_u64() && !n.is_i64()) {
        out.push(70);
        out.extend_from_slice(&f.to_be_bytes());
        return;
    }
    if let Some(u) = n.as_u64() {
        if u <= u8::MAX as u64 {
            out.push(97);
            out.push(u as u8);
        } else if u <= i32::MAX as u64 {
            out.push(98);
            out.extend_from_slice(&(u as i32).to_be_bytes());
        } else {
            write_big(out, false, u);
        }
    } else if let Some(i) = n.as_i64() {
        if i >= i32::MIN as i64 {
            out.push(98);
            out.extend_from_slice(&(i as i32).to_be_bytes());
        } else {
            write_big(out, true, i.unsigned_abs());
        }
    }
}

fn write_big(out: &mut Vec<u8>, negative: bool, magnitude: u64) {
    let bytes = magnitude.to_le_bytes();
    let len = 8 - (magnitude.leading_zeros() / 8) as usize;
    out.push(110);
    out.push(len as u8);
    out.push(negative as u8);
    out.extend_from_slice(&bytes[..len]);
}

pub fn decode(data: &[u8]) -> BoxedResult<Value> {
    if data.first() != Some(&VERSION) {
        return Err("Invalid ETF version".into());
    }
    let mut reader = Reader { data, pos: 1 };
    reader.value()
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> BoxedResult<&[u8]> {
        let end = self.pos.checked_add(n).filter(|&e| e <= self.data.len());
        let end = end.ok_or_else(|| BoxedError::from("Truncated ETF"))?;
        let slice = &self.data[self.pos..end];
        self.pos = end;
        Ok(slice)
    }

    fn u8(&mut self) -> BoxedResult<u8> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> BoxedResult<usize> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into()?) as usize)
    }

    fn u32(&mut self) -> BoxedResult<usize> {
        Ok(u32::from_be_bytes(self.take(4)?.try_into()?) as usize)
    }

    fn string(&mut self, len: usize) -> BoxedResult<String> {
        Ok(String::from_utf8_lossy(self.take(len)?).into_owned())
    }

    fn atom(&mut self, name: String) -> Value {
        match name.as_str() {
            "nil" | "null" | "undefined" => Value::Null,
            "true" => Value::Bool(true),
            "false" => Value::Bool(false),
            _ => Value::String(name),
        }
    }

    fn big(&mut self, len: usize) -> BoxedResult<Value> {
        let negative = self.u8()? == 1;
        let digits = self.take(len)?;
        let mut magnitude: u128 = 0;
        for (i, b) in digits.iter().enumerate() {
            if i >= 16 {
                return Err("ETF integer too large".into());
            }
            magnitude |= (*b as u128) << (8 * i);
        }
        if negative {
            if let Ok(v) = i64::try_from(-(magnitude as i128)) {
                return Ok(Value::Number(v.into()));
            }
        } else if let Ok(v) = u64::try_from(magnitude) {
            if v <= MAX_SAFE_INTEGER {
                return Ok(Value::Number(v.into()));
            }
            return Ok(Value::String(v.to_string()));
        }
        Ok(Value::String(format!(
            "{}{}",
            if negative { "-" } else { "" },
            magnitude
        )))
    }

    fn value(&mut self) -> BoxedResult<Value> {
        let tag = self.u8()?;
        match tag {
            97 => Ok(Value::Number(self.u8()?.into())),
            98 => {
                let v = i32::from_be_bytes(self.take(4)?.try_into()?);
                Ok(Value::Number(v.into()))
            }
            70 => {
                let f = f64::from_be_bytes(self.take(8)?.try_into()?);
                Ok(Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or(Value::Null))
            }
            99 => {
                let raw = self.string(31)?;
                let f: f64 = raw.trim_end_matches('\0').parse()?;
                Ok(Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or(Value::Null))
            }
            100 | 118 => {
                let len = self.u16()?;
                let name = self.string(len)?;
                Ok(self.atom(name))
            }
            115 | 119 => {
                let len = self.u8()? as usize;
                let name = self.string(len)?;
                Ok(self.atom(name))
            }
            104 | 105 => {
                let arity = if tag == 104 {
                    self.u8()? as usize
                } else {
                    self.u32()?
                };
                let items = (0..arity)
                    .map(|_| self.value())
                    .collect::<BoxedResult<Vec<_>>>()?;
                Ok(Value::Array(items))
            }
            106 => Ok(Value::Array(Vec::new())),
            107 => {
                let len = self.u16()?;
                let bytes = self.take(len)?;
                Ok(Value::Array(
                    bytes.iter().map(|b| Value::from(*b)).collect(),
                ))
            }
            108 => {
                let len = self.u32()?;
                let mut items = Vec::with_capacity(len.min(4096));
                for _ in 0..len {
                    items.push(self.value()?);
                }
                if self.u8()? != 106 {
                    return Err("Improper ETF list".into());
                }
                Ok(Value::Array(items))
            }
            109 => {
                let len = self.u32()?;
                Ok(Value::String(self.string(len)?))
            }
            110 => {
                let len = self.u8()? as usize;
                self.big(len)
            }
            111 => {
                let len = self.u32()?;
                self.big(len)
            }
            116 => {
                let len = self.u32()?;
                let mut map = Map::with_capacity(len.min(4096));
                for _ in 0..len {
                    let key = match self.value()? {
                        Value::String(s) => s,
                        other => other.to_string(),
                    };
                    map.insert(key, self.value()?);
                }
                Ok(Value::Object(map))
            }
            other => Err(format!("Unsupported ETF tag {}", other).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn roundtrip() {
        let value = json!({
            "op": 2,
            "d": {
                "capabilities": 1734653,
                "big": 9007199254740000u64,
                "neg": -5,
                "flag": true,
                "off": false,
                "none": null,
                "list": [1, "a", []],
                "empty": {},
                "text": "héllo"
            }
        });
        assert_eq!(decode(&encode(&value)).unwrap(), value);
    }

    #[test]
    fn snowflakes_become_strings() {
        let decoded = decode(&encode(&json!({"id": 1234567890123456789u64}))).unwrap();
        assert_eq!(decoded, json!({"id": "1234567890123456789"}));
    }
}
