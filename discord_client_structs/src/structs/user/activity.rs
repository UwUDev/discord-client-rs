use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_option_string_to_u64,
    deserialize_string_to_u64,
};
use crate::structs::misc::Emoji;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Activity {
    pub name: String,
    pub r#type: u8,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamp>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
    pub id: Option<String>,
    pub buttons: Option<Vec<String>>,
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
pub struct EmbeddedActivityConfig {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub activity_preview_video_asset_id: u64,
    pub supported_platforms: Vec<String>,
    pub default_orientation_lock_state: u64,
    pub tablet_default_orientation_lock_state: u64,
    pub requires_age_gate: bool,
    pub premium_tier_requirement: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub free_period_starts_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub free_period_ends_at: Option<DateTime<Utc>>,
    pub client_platform_config: Option<EmbeddedActivityPlatformConfig>,
    pub shelf_rank: u64,
    pub has_csp_exception: bool,
    pub displays_advertisements: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbeddedActivityPlatformConfig {
    pub label_type: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub label_until: Option<DateTime<Utc>>,
    pub release_phase: String,
}
