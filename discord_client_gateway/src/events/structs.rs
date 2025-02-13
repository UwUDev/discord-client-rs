use discord_client_strucs::deserializer::{
    deserialize_option_string_to_u64, deserialize_string_to_u64,
};
use discord_client_strucs::structs::{Activity, Emoji, Member, Message, User};
use serde::Deserialize;
use serde_json::Value;

// Event list
#[derive(Debug, Clone)]
pub enum Event {
    Ready(ReadyEvent),
    MessageCreate(MessageCreateEvent),
    MessageReactionAdd(MessageReactionAddEvent),
    GatewayReconnect(GatewayReconnectEvent),
    PresenceUpdate(PresenceUpdateEvent),
    Unknown(UnknownEvent),
}

// Gateway event payload
#[derive(Debug, Deserialize, Clone)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
    pub t: Option<String>,
}

/// Event structs ///

#[derive(Debug, Deserialize, Clone)]
pub struct ReadyEvent {
    pub session_id: String,
    pub analytics_token: String,
    pub auth_session_id_hash: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageCreateEvent {
    #[serde(flatten)]
    pub base: Message,
    pub channel_type: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub member: Option<Member>,
    #[serde(rename = "mentions")]
    pub mentions_with_members: Vec<UserWithMember>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageUpdateEvent {
    #[serde(flatten)]
    pub base: Message,
    pub channel_type: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub member: Option<Member>,
    #[serde(rename = "mentions")]
    pub mentions_with_members: Vec<UserWithMember>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserWithMember {
    #[serde(flatten)]
    pub user: User,
    pub member: Option<Member>,
}


#[derive(Debug, Deserialize, Clone)]
pub struct MessageReactionAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    #[serde(default)]
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

#[derive(Debug, Deserialize, Clone)]
pub struct PresenceUpdateEvent {
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: Option<bool>,
}

/// Misc events ///

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayReconnectEvent;

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub event_type: String,
    pub data: Value,
    pub op: u8,
}
