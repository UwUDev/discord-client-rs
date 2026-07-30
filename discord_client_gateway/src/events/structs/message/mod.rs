use discord_client_macros::{Flags, discord_struct};
use discord_client_structs::deserializer::*;
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::user::{Member, User};
use serde::Deserialize;

pub mod mention;
pub mod poll;
pub mod reaction;

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct MessageCreateEvent {
    #[serde(flatten)]
    pub message: Message,
    #[serde(default)]
    pub channel_type: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub member: Option<Member>,
    #[serde(default)]
    #[serde(rename = "mentions")]
    pub mentions_with_members: Vec<UserWithMember>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct MessageUpdateEvent {
    #[serde(flatten)]
    pub message: Message,
    #[serde(default)]
    pub channel_type: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub member: Option<Member>,
    #[serde(default)]
    #[serde(rename = "mentions")]
    pub mentions_with_members: Vec<UserWithMember>,
}

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct MessageDeleteEvent {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub channel_id: u64,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageDeleteBulkEvent {
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub ids: Vec<u64>,
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

#[derive(Debug, Deserialize, Clone, Flags)]
pub struct MessageAckEvent {
    pub version: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    pub mention_count: Option<u32>,
    pub manual: Option<bool>,
    pub last_viewed: Option<u64>,
    pub flags: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub ack_type: Option<u8>,
}
