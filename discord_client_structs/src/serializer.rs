use chrono::{DateTime, Utc};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;

pub fn serialize_u64_as_string<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Conversion du nombre en chaîne
    let s = value.to_string();
    serializer.serialize_str(&s)
}

pub fn serialize_option_u64_as_string<S>(
    value: &Option<u64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => {
            let s = v.to_string();
            serializer.serialize_some(&s)
        }
        None => serializer.serialize_none(),
    }
}

pub fn serialize_vec_u64_as_string<S>(vec: &Vec<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Transformation de chaque élément en chaîne
    let string_vec: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
    string_vec.serialize(serializer)
}

pub fn serialize_option_vec_u64_as_string<S>(
    opt: &Option<Vec<u64>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt {
        Some(vec) => {
            let string_vec: Vec<String> = vec.iter().map(|n| n.to_string()).collect();
            serializer.serialize_some(&string_vec)
        }
        None => serializer.serialize_none(),
    }
}

pub fn serialize_date_to_iso8601_string<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.to_rfc3339();
    serializer.serialize_str(&s)
}

pub fn serialize_option_date_to_iso8601_string<S>(
    opt: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt {
        Some(date) => {
            let s = date.to_rfc3339();
            serializer.serialize_some(&s)
        }
        None => serializer.serialize_none(),
    }
}

pub fn serialize_map_of_u64_to_string<S>(
    map: &HashMap<u64, String>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let string_map: HashMap<String, String> = map
        .iter()
        .map(|(k, v)| (k.to_string(), v.clone()))
        .collect();
    string_map.serialize(serializer)
}

pub fn serialize_datetime_as_timestamp<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.timestamp_millis())
}
pub fn serialize_option_datetime_as_timestamp<S>(
    date: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serialize_datetime_as_timestamp(dt, serializer),
        None => serializer.serialize_none(),
    }
}
