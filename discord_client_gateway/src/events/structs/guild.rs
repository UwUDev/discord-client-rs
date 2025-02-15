use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::UpdatedChannel;
use discord_client_structs::structs::channel::voice::VoiceState;
use discord_client_structs::structs::user::Member;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PassiveUpdateV2Event {
    pub updated_voice_states: Vec<VoiceState>,
    pub updated_members: Vec<Member>,
    pub updated_channels: Vec<UpdatedChannel>,
    pub removed_voice_states: Vec<serde_json::Value>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}
