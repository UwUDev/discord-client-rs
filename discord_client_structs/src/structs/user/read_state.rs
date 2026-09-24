use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct ReadState {
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    #[snowflake]
    pub last_message_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    pub mention_count: Option<u32>,
    pub read_state_type: Option<u8>,
    pub flags: Option<u64>,
    pub badge_count: Option<u32>,
}
