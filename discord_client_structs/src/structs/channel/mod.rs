use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_option_string_to_u64,
    deserialize_option_string_to_vec_u64, deserialize_string_to_u64,
};
use crate::structs::channel::thread::{DefaultReaction, Tag, ThreadMember, ThreadMetadata};
use crate::structs::permission::Overwrite;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use serde::Deserialize;

pub mod summary;
pub mod thread;
pub mod voice;

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub channel_type: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub position: Option<i64>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub last_message_id: Option<u64>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u16>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub owner_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub managed: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub parent_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u64>,
    pub total_message_sent: Option<u32>,
    pub available_tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub applied_tags: Option<Vec<u64>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<u32>,
    pub default_sort_order: Option<u8>,
    pub default_forum_layout: Option<u8>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PartialChannel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub name: Option<String>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdatedChannel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub last_message_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
}
