use crate::deserializer::*;
use crate::structs::application::IntegrationApplication;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use discord_client_macros::CreatedAt;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct Integration {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_integration_type")]
    #[serde(serialize_with = "serialize_integration_type")]
    pub r#type: IntegrationType,
    pub enabled: bool,
    pub account: IntegrationAccount,
    pub syncing: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub role_id: Option<u64>,
    pub enable_emoticons: Option<bool>,
    pub expire_behavior: Option<u32>,
    pub expire_grace_period: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub synced_at: Option<DateTime<Utc>>,
    pub subscriber_count: Option<u32>,
    pub revoked: Option<bool>,
    pub application: Option<IntegrationApplication>,
    pub scopes: Option<Vec<String>>,
    pub role_connections_metadata: Option<Vec<ApplicationRoleConnectionMetadata>>,
    pub user: Option<User>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationAccount {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationRoleConnectionMetadata {
    #[serde(deserialize_with = "deserialize_application_role_connection_metadata_type")]
    pub r#type: RoleConnectionOperatorType,
    pub key: String,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrationType {
    Twitch,
    YouTube,
    Discord,
    GuildSubscription,
    Unknown(String),
}

impl IntegrationType {
    pub fn as_str(&self) -> &str {
        match self {
            IntegrationType::Twitch => "twitch",
            IntegrationType::YouTube => "youtube",
            IntegrationType::Discord => "discord",
            IntegrationType::GuildSubscription => "guild_subscription",
            IntegrationType::Unknown(s) => s,
        }
    }
}

impl From<&str> for IntegrationType {
    fn from(s: &str) -> Self {
        match s {
            "twitch" => IntegrationType::Twitch,
            "youtube" => IntegrationType::YouTube,
            "discord" => IntegrationType::Discord,
            "guild_subscription" => IntegrationType::GuildSubscription,
            _ => IntegrationType::Unknown(s.to_string()),
        }
    }
}

impl Default for IntegrationType {
    fn default() -> Self {
        IntegrationType::Unknown("unknown".to_string())
    }
}

pub(crate) fn deserialize_integration_type<'de, D>(
    deserializer: D,
) -> Result<IntegrationType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(IntegrationType::from(s.as_str()))
}

pub(crate) fn serialize_integration_type<S>(
    integration_type: &IntegrationType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(integration_type.as_str())
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RoleConnectionOperatorType {
    IntegerLessThanOrEqual,
    IntegerGreaterThanOrEqual,
    IntegerEqual,
    IntegerNotEqual,
    DateTimeLessThanOrEqual,
    DateTimeGreaterThanOrEqual,
    BooleanEqual,
    BooleanNotEqual,
    Unknown(u8),
}

impl RoleConnectionOperatorType {
    pub fn as_u8(&self) -> u8 {
        match self {
            RoleConnectionOperatorType::IntegerLessThanOrEqual => 1,
            RoleConnectionOperatorType::IntegerGreaterThanOrEqual => 2,
            RoleConnectionOperatorType::IntegerEqual => 3,
            RoleConnectionOperatorType::IntegerNotEqual => 4,
            RoleConnectionOperatorType::DateTimeLessThanOrEqual => 5,
            RoleConnectionOperatorType::DateTimeGreaterThanOrEqual => 6,
            RoleConnectionOperatorType::BooleanEqual => 7,
            RoleConnectionOperatorType::BooleanNotEqual => 8,
            RoleConnectionOperatorType::Unknown(value) => *value,
        }
    }
}

impl From<u8> for RoleConnectionOperatorType {
    fn from(value: u8) -> Self {
        match value {
            1 => RoleConnectionOperatorType::IntegerLessThanOrEqual,
            2 => RoleConnectionOperatorType::IntegerGreaterThanOrEqual,
            3 => RoleConnectionOperatorType::IntegerEqual,
            4 => RoleConnectionOperatorType::IntegerNotEqual,
            5 => RoleConnectionOperatorType::DateTimeLessThanOrEqual,
            6 => RoleConnectionOperatorType::DateTimeGreaterThanOrEqual,
            7 => RoleConnectionOperatorType::BooleanEqual,
            8 => RoleConnectionOperatorType::BooleanNotEqual,
            _ => RoleConnectionOperatorType::Unknown(value),
        }
    }
}

fn deserialize_application_role_connection_metadata_type<'de, D>(
    deserializer: D,
) -> Result<RoleConnectionOperatorType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u8::deserialize(deserializer)?;
    Ok(RoleConnectionOperatorType::from(value))
}
