use crate::deserializer::*;
use chrono::{DateTime, Utc};
use discord_client_macros::CreatedAt;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct ChannelUnreadUpdate {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub last_message_id: Option<u64>,
}
