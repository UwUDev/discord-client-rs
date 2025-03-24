use discord_client_macros::CreatedAt;
use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::voice::VoiceState;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceStateUpdateEvent {
    #[serde(flatten)]
    pub voice_state: VoiceState,
}

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct VoiceChannelStatusUpdateEvent {
    pub status: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}
