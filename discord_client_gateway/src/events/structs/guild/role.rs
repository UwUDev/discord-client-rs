use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::guild::role::Role;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildRoleCreateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub role: Role,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildRoleUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub role: Role,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildRoleDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub role_id: u64,
}
