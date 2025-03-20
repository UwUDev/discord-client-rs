use crate::deserializer::*;
use crate::structs::application::IntegrationApplication;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Integration {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub r#type: String, // todo: enum
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
    pub r#type: u8, // todo: enum
    pub key: String,
    pub name: String,
    pub name_localizations: Option<HashMap<String, String>>,
    pub description: String,
    pub description_localizations: Option<HashMap<String, String>>,
}
