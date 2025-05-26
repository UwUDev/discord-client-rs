use discord_client_structs::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DirectMessageSettingsUpsellShowEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}
