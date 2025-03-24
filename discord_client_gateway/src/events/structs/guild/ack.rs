use discord_client_structs::deserializer::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildFeatureAckEvent {
    pub version: u32,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub resource_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub entity_id: u64,
    pub ack_type: u8,
}
