use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::message::sticker::Sticker;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildStickersUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub stickers: Vec<Sticker>,
}
