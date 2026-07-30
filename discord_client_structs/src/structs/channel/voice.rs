use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;
use serde::Deserialize;

#[discord_struct]
pub struct VoiceState {
    #[snowflake]
    pub user_id: u64,
    pub suppress: bool,
    pub session_id: String,
    pub self_video: bool,
    pub self_mute: bool,
    pub self_deaf: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub request_to_speak_timestamp: Option<DateTime<Utc>>,
    pub mute: bool,
    pub deaf: bool,
    #[snowflake]
    pub channel_id: Option<u64>, // null on left voice channel event
}

#[derive(Debug, Deserialize, Clone)]
pub struct Stream {
    pub stream_key: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub rtc_server_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub rtc_channel_id: Option<u64>,
    pub region: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub viewer_ids: Vec<u64>,
    pub paused: Option<bool>,
}

#[cfg(test)]
mod stream_tests {
    use super::Stream;

    #[test]
    fn parses_stream_create_shape() {
        let raw = r#"{"stream_key":"guild:1:2:3","rtc_server_id":"4","region":"paris","viewer_ids":[],"paused":false}"#;
        let s: Stream = serde_json::from_str(raw).unwrap();
        assert_eq!(s.stream_key, "guild:1:2:3");
        assert_eq!(s.rtc_server_id, Some(4));
        assert_eq!(s.region.as_deref(), Some("paris"));
        assert!(s.viewer_ids.is_empty());
        assert_eq!(s.paused, Some(false));
    }
}
