use discord_client_structs::deserializer::{
    deserialize_option_string_to_u64, deserialize_string_to_u64,
};
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::misc::Emoji;
use discord_client_structs::structs::user::{Member, User};
use serde::Deserialize;

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
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_author_id: Option<u64>,
    pub burst: bool,
    pub burst_colors: Option<Vec<String>>,
    #[serde(rename = "type")]
    pub reaction_type: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserWithMember {
    #[serde(flatten)]
    pub user: User,
    pub member: Option<Member>,
}
