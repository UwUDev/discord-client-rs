use chrono::{DateTime, Utc};
use discord_client_structs::deserializer::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AutoModMentionRaidDetectionEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub decision_id: String,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub suspicious_mention_activity_until: Option<DateTime<Utc>>,
}
