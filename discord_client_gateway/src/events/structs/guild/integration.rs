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

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}
