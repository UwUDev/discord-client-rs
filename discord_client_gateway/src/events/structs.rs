use discord_client_strucs::deserializer::{deserialize_option_string_to_u64, deserialize_string_to_u64};
use discord_client_strucs::structs::{Emoji, Member};
use serde::Deserialize;
use serde_json::Value;

// Event list
#[derive(Debug)]
pub enum Event {
    Ready(ReadyEvent),
    MessageCreate(MessageCreateEvent),
    MessageReactionAdd(MessageReactionAddEvent),
    Unknown(UnknownEvent),
}

// Gateway event payload
#[derive(Debug, Deserialize)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

/// Event structs ///

#[derive(Debug, Deserialize)]
pub struct ReadyEvent {
    pub session_id: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageCreateEvent {
    pub content: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
}

#[derive(Debug, Deserialize)]
pub struct MessageReactionAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub member: Option<Member>,
    pub emoji: Emoji,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_author_id: Option<u64>,
    pub burst: bool,
    pub burst_colors: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub reaction_type: u8,
}

#[derive(Debug)]
pub struct UnknownEvent {
    pub event_type: String,
    pub data: Value,
    pub op: u8,
}
