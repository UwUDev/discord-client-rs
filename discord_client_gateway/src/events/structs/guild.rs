use discord_client_structs::deserializer::deserialize_option_iso8601_string_to_date;
use discord_client_structs::deserializer::deserialize_iso8601_string_to_date;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_vec_u64;
use discord_client_structs::structs::channel::UpdatedChannel;
use discord_client_structs::structs::channel::voice::VoiceState;
use discord_client_structs::structs::user::{AvatarDecorationData, Member, User};
use serde::Deserialize;
use chrono::{DateTime, Utc};

#[derive(Debug, Deserialize, Clone)]
pub struct PassiveUpdateV2Event {
    pub updated_voice_states: Vec<VoiceState>,
    pub updated_members: Vec<Member>,
    pub updated_channels: Vec<UpdatedChannel>,
    pub removed_voice_states: Vec<serde_json::Value>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub user: User,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub roles: Vec<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub joined_at: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: Option<bool>,
    pub mute: Option<bool>,
    pub pending: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub communication_disabled_until: Option<DateTime<Utc>>,
    pub flags: u64,
}