use crate::deserializer::deserialize_string_to_vec_u64;
use crate::structs::channel::deserialize_string_to_u64;
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct Summary {
    #[serde(rename = "unsafe")]
    pub is_unsafe: bool,
    #[serde(rename = "type")]
    pub summary_type: u8,
    pub topic: String,
    pub summ_short: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub start_id: u64,
    pub source: u8,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub people: Vec<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub end_id: u64,
    pub count: u32,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub message_ids: Vec<u64>,
}
