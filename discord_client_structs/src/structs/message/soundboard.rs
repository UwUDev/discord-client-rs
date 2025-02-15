use crate::deserializer::deserialize_option_string_to_u64;
use crate::structs::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SoundboardSound {
    pub sound_id: u64,
    pub name: String,
    pub volume: f64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub available: bool,
    pub user: Option<User>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
}
