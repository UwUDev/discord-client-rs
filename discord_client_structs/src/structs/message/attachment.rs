use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_option_string_to_u64,
    deserialize_option_string_to_vec_u64, deserialize_string_to_u64,
};
use crate::structs::application::IntegrationApplication;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub filename: String,
    pub title: Option<String>,
    pub uploaded_filename: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub flags: Option<u64>,
    pub is_clip: Option<bool>,
    pub is_thumbnail: Option<bool>,
    pub is_remix: Option<bool>,
    pub is_spoiler: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub clip_created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub clip_participant_ids: Option<Vec<u64>>,
    pub clip_participants: Option<Vec<User>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub application: Option<IntegrationApplication>,
}
