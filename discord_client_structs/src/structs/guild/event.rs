use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::EntityMetadata;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct GuildScheduledEvent {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub guild_id: u64,
    #[serde(default)]
    #[snowflake]
    pub channel_id: Option<u64>,
    #[serde(default)]
    #[snowflake]
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
    #[snowflake]
    pub entity_id: Option<u64>,
    pub entity_metadata: Option<EntityMetadata>,
    pub user_count: Option<u32>,
    pub image: Option<String>,
    pub recurrence_rule: Option<RecurrenceRule>,
    pub guild_scheduled_event_exceptions: Vec<GuildScheduledEventException>,
}

#[discord_struct]
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

#[discord_struct]
pub struct RecurrenceRuleNWeekday {
    pub n: u8,
    pub day: u8,
}

#[discord_struct]
pub struct GuildScheduledEventException {
    #[snowflake]
    pub event_id: u64,
    #[snowflake]
    pub event_exception_id: u64,
    pub is_canceled: bool,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub scheduled_start_time: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub scheduled_end_time: Option<DateTime<Utc>>,
}
