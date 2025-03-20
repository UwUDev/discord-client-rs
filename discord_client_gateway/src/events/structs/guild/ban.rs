use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildBanAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub user: User,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildBanRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub user: User,
}
