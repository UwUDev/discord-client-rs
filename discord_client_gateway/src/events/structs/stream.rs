use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::structs::channel::voice::Stream;
use serde::Deserialize;

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
