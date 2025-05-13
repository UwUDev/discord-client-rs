use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Deserializer, de};

pub fn deserialize_string_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse::<u64>().map_err(de::Error::custom)
}

pub fn deserialize_option_string_to_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => s.parse::<u64>().map(Some).map_err(de::Error::custom),
        None => Ok(None),
    }
}

pub fn deserialize_string_to_vec_u64<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<String> = Vec::<String>::deserialize(deserializer)?;
    v.into_iter()
        .map(|s| s.parse::<u64>().map_err(de::Error::custom))
        .collect()
}

pub fn deserialize_option_string_to_vec_u64<'de, D>(
    deserializer: D,
) -> Result<Option<Vec<u64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<Vec<String>>::deserialize(deserializer)?;
    match opt {
        Some(v) => v
            .into_iter()
            .map(|s| s.parse::<u64>().map_err(de::Error::custom))
            .collect::<Result<Vec<u64>, _>>()
            .map(Some),
        None => Ok(None),
    }
}

pub fn deserialize_iso8601_string_to_date<'de, D>(
    deserializer: D,
) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(de::Error::custom)
}

pub fn deserialize_option_iso8601_string_to_date<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    match opt {
        Some(s) => DateTime::parse_from_rfc3339(&s)
            .map(|dt| Some(dt.with_timezone(&Utc)))
            .map_err(de::Error::custom),
        None => Ok(None),
    }
}

pub fn deserialize_map_of_u64_string<'de, D>(
    deserializer: D,
) -> Result<std::collections::HashMap<u64, String>, D::Error>
where
    D: Deserializer<'de>,
{
    let map: std::collections::HashMap<String, String> =
        std::collections::HashMap::<String, String>::deserialize(deserializer)?;
    let mut new_map = std::collections::HashMap::new();
    for (k, v) in map {
        new_map.insert(k.parse::<u64>().map_err(de::Error::custom)?, v);
    }
    Ok(new_map)
}

pub fn deserialize_timestamp_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = i64::deserialize(deserializer)?;
    Utc.timestamp_millis_opt(timestamp)
        .single()
        .ok_or_else(|| de::Error::custom("Invalid timestamp"))
}

pub fn deserialize_option_timestamp_to_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<i64>::deserialize(deserializer)?;
    match opt {
        Some(timestamp) => Ok(Some(
            Utc.timestamp_millis_opt(timestamp)
                .single()
                .ok_or_else(|| de::Error::custom("Invalid timestamp"))?,
        )),
        None => Ok(None),
    }
}
