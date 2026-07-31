use crate::deserializer::*;
use crate::serializer::*;
use chrono::{DateTime, Utc};
use discord_client_macros::{EnumFromPrimitive, discord_struct};

#[discord_struct]
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

#[discord_struct]
pub struct AutomodRule {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub guild_id: u64,
    pub name: String,
    #[snowflake]
    pub creator_id: u64,
    pub event_type: AutomodEventType,
    pub trigger_type: AutomodTriggerType,
    pub trigger_metadata: TriggerMetadata,
    pub actions: Vec<Action>,
    pub enabled: bool,
    pub exempt_roles: Vec<u64>,
    pub exempt_channels: Vec<u64>,
}

#[discord_struct]
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

#[discord_struct]
pub struct Action {
    pub r#type: AutomodActionType,
    pub metadata: Option<ActionMetadata>,
}

#[discord_struct]
pub struct ActionMetadata {
    #[serde(default)]
    #[snowflake]
    pub channel_id: Option<u64>,
    pub duration_seconds: Option<u32>,
    pub custom_message: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum AutomodEventType {
    #[default]
    MessageSend = 1,
    GuildMemberEvent = 2,
    Unknown(u16),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum AutomodTriggerType {
    #[default]
    Keyword = 1,
    HarmfulLink = 2,
    Spam = 3,
    KeywordPreset = 4,
    MentionSpam = 5,
    UserProfile = 6,
    GuildPolicy = 7,
    Unknown(u16),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum AutomodActionType {
    BlockMessage = 1,
    SendAlertMessage = 2,
    TimeoutUser = 3,
    QuarantineUser = 4,
    Unknown(u16),
}
