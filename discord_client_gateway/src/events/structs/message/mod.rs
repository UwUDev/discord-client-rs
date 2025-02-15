use discord_client_structs::deserializer::{
    deserialize_option_string_to_u64, deserialize_string_to_u64,
};
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::user::{Member, User};
use serde::Deserialize;

pub mod poll;
pub mod reaction;

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
