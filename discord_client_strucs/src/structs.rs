use crate::deserializer::{
    deserialize_iso8601_string_to_date, deserialize_option_iso8601_string_to_date,
    deserialize_option_string_to_u64, deserialize_option_string_to_vec_u64,
    deserialize_string_to_u64, deserialize_string_to_vec_u64,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
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

#[derive(Debug, Deserialize, Clone)]
pub struct Member {
    pub user: User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub roles: Vec<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub joined_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
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
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: u8,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamp>,
    pub application_id: Option<String>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
    pub id: Option<String>,
    //pub buttons: Option<Vec<ActivityButton>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityTimestamp {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Party {
    pub id: Option<String>,
    pub size: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityAsset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub instanced_match: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityButton {
    pub label: String,
    pub url: String,
}