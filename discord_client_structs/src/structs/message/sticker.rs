use crate::deserializer::{deserialize_option_string_to_u64, deserialize_string_to_u64};
use crate::structs::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StickerItem {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub format_type: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Sticker {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub pack_id: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub sticker_type: u8,
    pub format_type: u8,
    pub available: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub user: Option<User>,
    pub sort_value: Option<u64>,
}
