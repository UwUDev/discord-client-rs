use chrono::{DateTime, Utc};
use discord_client_structs::deserializer::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelPinsUpdateEvent {
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub last_pin_timestamp: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelPinsAckEvent {
    pub version: u32,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
}
