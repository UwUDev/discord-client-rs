use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::misc::Emoji;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Activity {
    pub name: String,
    pub r#type: u8,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamp>,
    #[serde(default)]
    #[snowflake]
    pub application_id: Option<u64>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    #[flag_enum(
        "Instance=0,Join=1,Spectate=2,JoinRequest=3,Sync=4,Play=5,PartyPrivacyFriends=6,PartyPrivacyVoiceChannel=7,Embedded=8"
    )]
    pub flags: Option<u64>,
    pub id: Option<String>,
    pub buttons: Option<Vec<String>>,
}

#[discord_struct]
pub struct ActivityTimestamp {
    #[serde(
        default,
        deserialize_with = "deserialize_option_timestamp_to_datetime",
        serialize_with = "serialize_option_datetime_as_timestamp"
    )]
    pub start: Option<DateTime<Utc>>,

    #[serde(
        default,
        deserialize_with = "deserialize_option_timestamp_to_datetime",
        serialize_with = "serialize_option_datetime_as_timestamp"
    )]
    pub end: Option<DateTime<Utc>>,
}

#[discord_struct]
pub struct Party {
    pub id: Option<String>,
    pub size: Option<Vec<u64>>,
}

#[discord_struct]
pub struct ActivityAsset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[discord_struct]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub instanced_match: Option<String>,
}

#[discord_struct]
pub struct EmbeddedActivityConfig {
    #[snowflake]
    pub application_id: u64,
    #[snowflake]
    pub activity_preview_video_asset_id: u64,
    pub supported_platforms: Vec<String>,
    pub default_orientation_lock_state: u64,
    pub tablet_default_orientation_lock_state: u64,
    pub requires_age_gate: bool,
    pub premium_tier_requirement: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub free_period_starts_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub free_period_ends_at: Option<DateTime<Utc>>,
    pub client_platform_config: Option<EmbeddedActivityPlatformConfig>,
    pub shelf_rank: u64,
    pub has_csp_exception: bool,
    pub displays_advertisements: bool,
}

#[discord_struct]
pub struct EmbeddedActivityPlatformConfig {
    pub label_type: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub label_until: Option<DateTime<Utc>>,
    pub release_phase: String,
}
