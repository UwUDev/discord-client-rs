use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_string_to_vec_u64,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MessageCall {
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub participants: Vec<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub ended_timestamp: Option<DateTime<Utc>>,
}
