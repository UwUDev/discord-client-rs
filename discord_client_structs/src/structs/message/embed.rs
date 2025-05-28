use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::{EnumFromString, Flags};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct Embed {
    pub title: Option<String>,
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
    #[flag_enum("ContainsExplicitMedia=4,ContentInventoryEntry=5")]
    pub flags: Option<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromString, Default)]
pub enum EmbedType {
    #[default]
    Rich,
    Image,
    Video,
    Gifv,
    Article,
    Link,
    #[str_value("poll_result")]
    PollResult,
    Unknown,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
//noinspection DuplicatedCode
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    #[flag_enum(
        "IsClip=0,IsThumbnail=1,IsRemix=2,IsSpoiler=3,ContainsExplicitMedia=4,IsAnimated=5"
    )]
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
//noinspection DuplicatedCode
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    #[flag_enum(
        "IsClip=0,IsThumbnail=1,IsRemix=2,IsSpoiler=3,ContainsExplicitMedia=4,IsAnimated=5"
    )]
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
//noinspection DuplicatedCode
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    #[flag_enum(
        "IsClip=0,IsThumbnail=1,IsRemix=2,IsSpoiler=3,ContainsExplicitMedia=4,IsAnimated=5"
    )]
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
