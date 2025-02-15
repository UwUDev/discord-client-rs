use crate::deserializer::{
    deserialize_iso8601_string_to_date, deserialize_option_iso8601_string_to_date,
    deserialize_string_to_u64, deserialize_string_to_vec_u64,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod activity;
pub mod session;

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u64>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    pub flags: Option<u64>,
    pub premium_type: Option<u64>,
    pub public_flags: Option<u64>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub roles: Vec<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub joined_at: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub communication_disabled_until: Option<DateTime<Utc>>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AvatarDecorationData {
    pub asset: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sku_id: u64,
}
