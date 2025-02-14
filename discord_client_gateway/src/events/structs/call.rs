use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_vec_u64;
use discord_client_structs::structs::channel::voice::VoiceState;
use discord_client_structs::structs::user::activity::EmbeddedActivityConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CallCreateEvent {
    pub voice_states: Vec<VoiceState>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub ringing: Vec<u64>,
    pub region: String,
    pub message_id: String,
    pub embedded_activities: Vec<EmbeddedActivityConfig>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
}
