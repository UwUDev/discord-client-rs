use crate::deserializer::{
    deserialize_iso8601_string_to_date, deserialize_option_string_to_u64,
    deserialize_option_string_to_vec_u64, deserialize_string_to_u64,
};
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Potion {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub used_by: u64,
    pub r#type: u8,
    pub emoji: Vec<Emoji>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}
