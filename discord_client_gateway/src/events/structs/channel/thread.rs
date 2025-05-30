use discord_client_macros::CreatedAt;
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

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct ThreadDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub parent_id: Option<u64>,
    pub r#type: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadListSyncEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub channel_ids: Option<Vec<u64>>,
    pub threads: Vec<Channel>,
    #[serde(default)]
    pub members: Option<Vec<ThreadMember>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadMemberUpdateEvent {
    #[serde(flatten)]
    pub thread_member: ThreadMember,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct ThreadMembersUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub member_ids_preview: Option<Vec<u64>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub removed_member_ids: Option<Vec<u64>>,
    pub member_count: u32,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    pub added_members: Option<Vec<ThreadMember>>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}
