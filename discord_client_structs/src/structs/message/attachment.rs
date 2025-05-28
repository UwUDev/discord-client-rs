use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::application::IntegrationApplication;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::{CreatedAt, Flags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct Attachment {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
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
    #[flag_enum(
        "IsClip=0,IsThumbnail=1,IsRemix=2,IsSpoiler=3,ContainsExplicitMedia=4,IsAnimated=5"
    )]
    pub flags: Option<u64>,
    pub is_clip: Option<bool>,
    pub is_thumbnail: Option<bool>,
    pub is_remix: Option<bool>,
    pub is_spoiler: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub clip_created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_option_vec_u64_as_string")]
    pub clip_participant_ids: Option<Vec<u64>>,
    pub clip_participants: Option<Vec<User>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub application_id: Option<u64>,
    pub application: Option<IntegrationApplication>,
}
