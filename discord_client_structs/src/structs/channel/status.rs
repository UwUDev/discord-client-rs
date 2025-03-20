use crate::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelStatus {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub status: String,
}
