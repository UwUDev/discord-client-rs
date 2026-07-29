use discord_client_structs::deserializer::{
    deserialize_option_string_to_u64, deserialize_string_to_vec_u64,
};
use serde::Deserialize;

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

#[derive(Debug, Deserialize, Clone)]
pub struct StreamCreateEvent {
    #[serde(flatten)]
    pub stream: Stream,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamUpdateEvent {
    #[serde(flatten)]
    pub stream: Stream,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamServerUpdateEvent {
    pub token: String,
    pub stream_key: String,
    pub endpoint: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StreamDeleteEvent {
    pub stream_key: String,
    pub reason: String,
    #[serde(default)]
    pub unavailable: Option<bool>,
}
