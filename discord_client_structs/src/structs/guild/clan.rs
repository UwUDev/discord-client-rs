use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::activity::GameActivity;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Clan {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub tag: String,
    pub icon_hash: Option<String>,
    pub member_count: u32,
    pub description: Option<String>,
    pub play_style: Option<u8>,
    pub search_terms: Option<Vec<String>>,
    pub game_application_ids: Option<Vec<u64>>,
    pub badge: Option<u8>,
    pub badge_hash: Option<String>,
    pub badge_color_primary: Option<String>,
    pub badge_color_secondary: Option<String>,
    pub banner: Option<u8>,
    pub banner_hash: Option<String>,
    pub brand_color_primary: Option<String>,
    pub brand_color_secondary: Option<String>,
    pub wildcard_descriptors: Option<Vec<String>>,
    pub game_activity: Option<Vec<GameActivity>>,
    pub discovery_profile_features: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct GuildJoinRequest {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub join_request_id: u64,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub created_at: DateTime<Utc>,
    pub application_status: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub guild_id: u64,
    #[serde(default)]
    pub form_responses: Option<Vec<MemberVerificationFormField>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub actioned_at: Option<u64>,
    #[serde(default)]
    pub actioned_by_user: Option<User>,
    #[serde(default)]
    pub rejection_reason: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub user_id: u64,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub interview_channel_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MemberVerificationFormField {
    pub field_type: String,
    pub label: String,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
    #[serde(default)]
    pub values: Option<Vec<String>>,
    #[serde(default)]
    pub response: Option<serde_json::Value>,
    pub required: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub automations: Option<Vec<String>>,
    #[serde(default)]
    pub placeholder: Option<String>,
}
