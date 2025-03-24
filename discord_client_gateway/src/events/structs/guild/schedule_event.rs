use discord_client_structs::deserializer::*;
use discord_client_structs::structs::guild::event::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventCreateEvent {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventUpdateEvent {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventDeleteEvent {
    #[serde(flatten)]
    pub event: GuildScheduledEvent,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventExceptionCreateEvent {
    #[serde(flatten)]
    pub exception: GuildScheduledEventException,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventExceptionUpdateEvent {
    #[serde(flatten)]
    pub exception: GuildScheduledEventException,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventExceptionDeleteEvent {
    #[serde(flatten)]
    pub exception: GuildScheduledEventException,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventExceptionsDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub event_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventUserAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    pub response: u8,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_scheduled_event_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEventUserRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    pub response: u8,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_scheduled_event_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}
