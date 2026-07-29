use discord_client_structs::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationCommandPermission {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub r#type: u8,
    pub permission: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationCommandPermissionsUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub permissions: Vec<ApplicationCommandPermission>,
}
