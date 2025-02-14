use crate::deserializer::{
    deserialize_iso8601_string_to_date, deserialize_option_iso8601_string_to_date,
    deserialize_option_string_to_u64, deserialize_string_to_u64,
};
use crate::structs::user::Member;
use chrono::{DateTime, Utc};
use serde::Deserialize;
#[derive(Debug, Deserialize, Clone)]
pub struct ThreadMember {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub join_timestamp: DateTime<Utc>,
    pub flags: u64,
    pub member: Option<Member>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub archive_timestamp: DateTime<Utc>,
    pub locked: bool,
    pub invitable: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub create_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultReaction {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}
