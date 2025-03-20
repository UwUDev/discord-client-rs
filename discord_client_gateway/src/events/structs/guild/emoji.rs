use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::misc::Emoji;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildEmojisUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub emojis: Vec<Emoji>,
}
