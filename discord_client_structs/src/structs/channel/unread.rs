use crate::deserializer::*;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct ChannelUnreadUpdate {
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[snowflake]
    pub last_message_id: Option<u64>,
}
