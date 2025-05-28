use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::user::Member;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::OptionCreatedAt;
use discord_client_macros::{CreatedAt, Flags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, OptionCreatedAt, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct ThreadMember {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub user_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub join_timestamp: DateTime<Utc>,
    #[flag_enum("HasInteracted=0,AllMessages=1,OnlyMentions=2,NoMessages=3")]
    pub flags: u64,
    pub member: Option<Member>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub archive_timestamp: DateTime<Utc>,
    pub locked: bool,
    pub invitable: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub create_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct Tag {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct DefaultReaction {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}
