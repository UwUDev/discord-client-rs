use crate::deserializer::*;
use crate::structs::application::IntegrationApplication;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use discord_client_macros::{CreatedAt, EnumFromPrimitive, EnumFromString};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct Integration {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
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
    pub r#type: RoleConnectionOperatorType,
    pub key: String,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromString)]
#[str_value("discord")]
pub enum IntegrationType {
    Twitch,
    YouTube,
    Discord,
    #[str_value("guild_subscription")]
    GuildSubscription,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum RoleConnectionOperatorType {
    #[default]
    IntegerLessThanOrEqual = 1,
    IntegerGreaterThanOrEqual = 2,
    IntegerEqual = 3,
    IntegerNotEqual = 4,
    DateTimeLessThanOrEqual = 5,
    DateTimeGreaterThanOrEqual = 6,
    BooleanEqual = 7,
    BooleanNotEqual = 8,
    Unknown(u8),
}
