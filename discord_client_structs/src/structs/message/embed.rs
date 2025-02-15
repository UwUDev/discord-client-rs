use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_option_string_to_u64,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Embed {
    pub title: Option<String>,
    pub r#type: String,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub timestamp: Option<DateTime<Utc>>,
    pub color: Option<u64>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub reference_id: Option<u64>,
    pub flags: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
