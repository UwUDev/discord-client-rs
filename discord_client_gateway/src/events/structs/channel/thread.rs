use discord_client_macros::discord_struct;
use discord_client_structs::deserializer::{
    deserialize_option_string_to_vec_u64, deserialize_string_to_u64,
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

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct ThreadDeleteEvent {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub guild_id: Option<u64>,
    #[snowflake]
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

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct ThreadMembersUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub member_ids_preview: Option<Vec<u64>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub removed_member_ids: Option<Vec<u64>>,
    pub member_count: u32,
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    pub added_members: Option<Vec<ThreadMember>>,
    #[snowflake]
    pub guild_id: u64,
}
