use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use serde::Deserialize;
use discord_client_structs::structs::user::Member;

#[derive(Debug, Deserialize, Clone)]
pub struct TypingStartEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    pub timestamp: u64,
    pub member: Option<Member>,
}