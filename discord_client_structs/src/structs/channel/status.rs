use crate::deserializer::deserialize_string_to_u64;
use discord_client_macros::CreatedAt;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct ChannelStatus {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub status: String,
}
