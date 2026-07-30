use discord_client_macros::discord_struct;
use discord_client_structs::deserializer::*;
use discord_client_structs::structs::guild::integration::Integration;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildIntegrationsUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationCreateEvent {
    #[serde(flatten)]
    pub integration: Integration,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationUpdateEvent {
    #[serde(flatten)]
    pub integration: Integration,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct IntegrationDeleteEvent {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub guild_id: u64,
    #[serde(default)]
    #[snowflake]
    pub application_id: Option<u64>,
}
