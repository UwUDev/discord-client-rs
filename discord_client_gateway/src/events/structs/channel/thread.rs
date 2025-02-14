use discord_client_structs::deserializer::{
    deserialize_option_string_to_u64, deserialize_option_string_to_vec_u64,
    deserialize_string_to_u64,
};
use discord_client_structs::structs::channel::Channel;
use discord_client_structs::structs::channel::thread::ThreadMember;
use serde::Deserialize;

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
