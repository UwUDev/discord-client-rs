use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct MessagePollVoteAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub answer_id: i64,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct MessagePollVoteRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub answer_id: i64,
}
