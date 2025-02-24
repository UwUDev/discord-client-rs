use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_embed_type")]
    #[serde(serialize_with = "serialize_embed_type")]
    pub r#type: EmbedType,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
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
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub reference_id: Option<u64>,
    pub flags: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EmbedType {
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
    PollResult,
    Unknown(String),
}

impl EmbedType {
    pub fn as_str(&self) -> &str {
        match self {
            EmbedType::Rich => "rich",
            EmbedType::Image => "image",
            EmbedType::Video => "video",
            EmbedType::Gifv => "gifv",
            EmbedType::Article => "article",
            EmbedType::Link => "link",
            EmbedType::PollResult => "poll_result",
            EmbedType::Unknown(s) => s,
        }
    }
}

impl From<&str> for EmbedType {
    fn from(s: &str) -> Self {
        match s {
            "rich" => EmbedType::Rich,
            "image" => EmbedType::Image,
            "video" => EmbedType::Video,
            "gifv" => EmbedType::Gifv,
            "article" => EmbedType::Article,
            "link" => EmbedType::Link,
            "poll_result" => EmbedType::PollResult,
            _ => EmbedType::Unknown(s.to_string()),
        }
    }
}

impl Default for EmbedType {
    fn default() -> Self {
        EmbedType::Rich
    }
}

fn deserialize_embed_type<'de, D>(deserializer: D) -> Result<EmbedType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(EmbedType::from(s.as_str()))
}

fn serialize_embed_type<S>(embed_type: &EmbedType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(embed_type.as_str())
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
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

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}
