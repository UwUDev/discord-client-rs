use crate::structs::channel::*;
use crate::structs::user::User;
use crate::structs::user::connection::IntegrationGuild;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Webhook {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub type_: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub channel_id: u64,
    #[serde(default)]
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub application_id: u64,
    #[serde(default)]
    pub source_guild: Option<IntegrationGuild>,
    #[serde(default)]
    pub source_channel: Option<WebhookChannel>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct WebhookChannel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
}
