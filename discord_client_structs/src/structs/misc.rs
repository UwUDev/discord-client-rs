use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;
use serde::Deserialize;

#[discord_struct]
pub struct Potion {
    #[snowflake(no_created_at)]
    pub used_by: u64,
    pub r#type: u8,
    pub emoji: Vec<Emoji>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub created_at: DateTime<Utc>,
}

#[discord_struct]
pub struct Emoji {
    #[serde(default)]
    #[snowflake]
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_option_vec_u64_as_string")]
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[discord_struct]
pub struct UserSettingsProto {
    pub r#type: u64,
    pub proto: String,
}

#[discord_struct]
pub struct TutorialIndicators {
    pub indicators_suppressed: bool,
    pub indicators_confirmed: Vec<String>,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Versioned<T> {
    pub entries: Vec<T>,
    pub partial: bool,
    pub version: u64,
}

#[discord_struct]
pub struct MuteConfig {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(default)]
    pub selected_time_window: Option<i64>,
}
