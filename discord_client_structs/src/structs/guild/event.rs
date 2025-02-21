use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::EntityMetadata;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct GuildScheduledEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub creator_id: Option<u64>,
    pub creator: Option<User>,
    pub name: String,
    pub description: Option<String>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub scheduled_start_time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub scheduled_end_time: Option<DateTime<Utc>>,
    pub auto_start: Option<bool>,
    pub privacy_level: u8,
    pub status: u8,
    pub entity_type: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub entity_id: Option<u64>,
    pub entity_metadata: Option<EntityMetadata>,
    pub user_count: Option<u32>,
    pub image: Option<String>,
    pub recurrence_rule: Option<RecurrenceRule>,
    pub guild_scheduled_event_exceptions: Vec<GuildScheduledEventException>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct RecurrenceRule {
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub start: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub end: Option<DateTime<Utc>>,
    pub frequency: u8,
    pub interval: u8,
    pub by_weekday: Option<Vec<u8>>,
    pub by_n_weekday: Option<Vec<RecurrenceRuleNWeekday>>,
    pub by_month: Option<Vec<u8>>,
    pub by_month_day: Option<Vec<u8>>,
    pub by_year_day: Option<Vec<u8>>,
    pub count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct RecurrenceRuleNWeekday {
    pub n: u8,
    pub day: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct GuildScheduledEventException {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub event_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub event_exception_id: u64,
    pub is_canceled: bool,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub scheduled_end_time: Option<DateTime<Utc>>,
}
