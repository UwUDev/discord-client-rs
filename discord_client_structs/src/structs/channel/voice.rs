use crate::deserializer::deserialize_option_iso8601_string_to_date;
use crate::deserializer::deserialize_string_to_u64;
use crate::deserializer::deserialize_option_string_to_u64;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceState {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    pub suppress: bool,
    pub session_id: String,
    pub self_video: bool,
    pub self_mute: bool,
    pub self_deaf: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub mute: bool,
    pub deaf: bool,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>, // null on left voice channel event
}
