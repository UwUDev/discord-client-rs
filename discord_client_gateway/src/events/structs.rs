use discord_client_structs::deserializer::{
    deserialize_option_string_to_u64, deserialize_option_string_to_vec_u64,
    deserialize_string_to_u64,
};
use discord_client_structs::structs::channel::Channel;
use discord_client_structs::structs::channel::thread::ThreadMember;
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::misc::Emoji;
use discord_client_structs::structs::user::activity::Activity;
use discord_client_structs::structs::user::{Member, User};
use serde::Deserialize;
use serde_json::Value;

// Event list
#[derive(Debug, Clone)]
pub enum Event {
    // Ready events
    Ready(ReadyEvent),
    // message events
    MessageCreate(MessageCreateEvent),
    MessageReactionAdd(MessageReactionAddEvent),
    // gateway events
    GatewayReconnect(GatewayReconnectEvent),
    // presence events
    PresenceUpdate(PresenceUpdateEvent),
    // channel events
    ChannelCreate(ChannelCreateEvent),
    ChannelUpdate(ChannelUpdateEvent),
    ChannelDelete(ChannelDeleteEvent),
    // thread events
    ThreadCreate(ThreadCreateEvent),
    ThreadUpdate(ThreadUpdateEvent),
    ThreadDelete(ThreadDeleteEvent),
    ThreadListSync(ThreadListSyncEvent),
    // misc events
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
//noinspection DuplicatedCode
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
//noinspection DuplicatedCode
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
pub struct ChannelCreateEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelUpdateEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelDeleteEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadCreateEvent {
    #[serde(flatten)]
    pub channel: Channel,
    pub newly_created: Option<bool>,
    pub thread_member: Option<ThreadMember>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadUpdateEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub parent_id: Option<u64>,
    #[serde(rename = "type")]
    pub thread_type: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadListSyncEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub channel_ids: Option<Vec<u64>>,
    pub threads: Vec<Channel>,
    pub members: Vec<ThreadMember>,
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
