use crate::deserializer::*;
use crate::serializer::*;
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct AuditLogEntry {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub target_id: Option<u64>,
    #[serde(default)]
    pub changes: Option<Vec<AuditLogChange>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub user_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub action_type: u8,
    #[serde(default)]
    pub options: Option<AuditEntryInfo>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct AuditLogChange {
    #[serde(default)]
    // can be an object, array, string, number and even boolean 💀
    pub new_value: Option<serde_json::Value>,
    #[serde(default)]
    pub old_value: Option<serde_json::Value>,
    pub key: String,
}
#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct AuditEntryInfo {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub application_id: Option<u64>,
    #[serde(default)]
    pub auto_moderation_rule_name: Option<String>,
    #[serde(default)]
    pub auto_moderation_rule_trigger_type: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub count: Option<String>,
    #[serde(default)]
    pub delete_member_days: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub id: Option<u64>,
    #[serde(default)]
    pub integration_type: Option<String>,
    #[serde(default)]
    pub members_removed: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub message_id: Option<u64>,
    #[serde(default)]
    pub role_name: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    pub status: Option<String>,
}
