use serde::Deserialize;

use crate::deserializer::deserialize_string_to_u64;

#[derive(Debug, Deserialize, Clone)]
pub struct Overwrite {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub r#type: u8,
    pub allow: String,
    pub deny: String,
}
