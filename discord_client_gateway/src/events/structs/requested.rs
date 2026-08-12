use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::status::ChannelStatus;
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::user::Member;
use discord_client_structs::structs::user::presence::Presence;
use serde::Deserialize;
use serde_json::Value;

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

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberListUpdateEvent {
    #[serde(default)]
    pub id: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    pub member_count: u64,
    #[serde(default)]
    pub online_count: u64,
    #[serde(default)]
    pub groups: Vec<GuildMemberListGroup>,
    #[serde(default)]
    pub ops: Vec<GuildMemberListOperation>,
}

impl GuildMemberListUpdateEvent {
    pub fn synced_members(&self) -> impl Iterator<Item = &Member> {
        self.ops
            .iter()
            .filter(|operation| operation.op == "SYNC")
            .flat_map(|operation| operation.items.iter())
            .filter_map(|item| item.member.as_ref())
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberListGroup {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub count: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberListOperation {
    #[serde(default)]
    pub op: String,
    #[serde(default)]
    pub range: Option<[u64; 2]>,
    #[serde(default)]
    pub items: Vec<GuildMemberListItem>,
    #[serde(default)]
    pub index: Option<u64>,
    #[serde(default)]
    pub item: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberListItem {
    #[serde(default)]
    pub member: Option<Member>,
    #[serde(default)]
    pub group: Option<GuildMemberListGroup>,
}
