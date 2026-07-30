use crate::deserializer::*;
use crate::serializer::*;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Summary {
    #[serde(rename = "unsafe")]
    pub is_unsafe: bool,
    pub r#type: u8,
    pub topic: String,
    pub summ_short: String,
    #[snowflake]
    pub start_id: u64,
    pub source: u8,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_vec_u64_as_string")]
    pub people: Vec<u64>,
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub end_id: u64,
    pub count: u32,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_vec_u64_as_string")]
    pub message_ids: Vec<u64>,
}
