use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::user::Member;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct ThreadMember {
    #[serde(default)]
    #[snowflake]
    pub id: Option<u64>,
    #[serde(default)]
    #[snowflake]
    pub user_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub join_timestamp: DateTime<Utc>,
    #[flag_enum("HasInteracted=0,AllMessages=1,OnlyMentions=2,NoMessages=3")]
    pub flags: u64,
    pub member: Option<Member>,
}

#[discord_struct]
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

#[discord_struct]
pub struct Tag {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[snowflake]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[discord_struct]
pub struct DefaultReaction {
    #[serde(default)]
    #[snowflake]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}
