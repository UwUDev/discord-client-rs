use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::status::ChannelStatus;
use discord_client_structs::structs::message::Message;
use serde::Deserialize;
use discord_client_structs::structs::user::Member;
use discord_client_structs::structs::user::presence::Presence;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelStatusesEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub channels: Vec<ChannelStatus>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LastMessagesEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub messages: Vec<Message>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMembersChunkEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub members: Vec<Member>,
    pub chunk_index: u64,
    pub chunk_count: u64,
    pub not_found: Option<Vec<String>>,
    pub presences: Option<Vec<Presence>>,
    pub nonce: Option<String>,
}