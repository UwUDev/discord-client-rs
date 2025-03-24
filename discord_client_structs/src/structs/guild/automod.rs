use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct AutomodIncidentsData {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub raid_detected_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub dm_spam_detected_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub invites_disabled_until: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub dms_disabled_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct AutomodRule {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub guild_id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub creator_id: u64,
    pub event_type: u8,   // todo: enum
    pub trigger_type: u8, // todo: enum
    pub trigger_metadata: TriggerMetadata,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub exempt_roles: Vec<u64>,
    pub exempt_channels: Vec<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct TriggerMetadata {
    #[serde(default)]
    pub keyword_filter: Vec<String>,
    #[serde(default)]
    pub regex_patterns: Vec<String>,
    #[serde(default)]
    pub presets: Vec<u8>,
    #[serde(default)]
    pub allow_list: Vec<String>,
    #[serde(default)]
    pub mention_total_limit: u8,
    #[serde(default)]
    pub mention_raid_protection_enabled: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Action {
    pub action_type: u8, // todo: enum
    pub metadata: Option<ActionMetadata>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ActionMetadata {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub channel_id: u64,
    pub duration_seconds: u32,
    pub custom_message: Option<String>,
}
