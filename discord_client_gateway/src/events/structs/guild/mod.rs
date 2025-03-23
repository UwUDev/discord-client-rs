use chrono::{DateTime, Utc};
use discord_client_structs::deserializer::*;
use discord_client_structs::structs::channel::UpdatedChannel;
use discord_client_structs::structs::channel::voice::VoiceState;
use discord_client_structs::structs::guild::log::AuditLogEntry;
use discord_client_structs::structs::guild::{GatewayGuild, Guild, UnavailableGuild};
use discord_client_structs::structs::user::{AvatarDecorationData, Member, User};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub mod automod;
pub mod ban;
pub mod emoji;
pub mod integration;
pub mod role;
pub mod sticker;

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

#[derive(Debug, Deserialize, Clone)]
pub struct GuildMemberAddEvent {
    #[serde(flatten)]
    pub member: Member,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Clone)]
pub struct GuildMemberRemoveEvent {
    pub guild_id: u64,
    pub user_id: u64,
}

impl<'de> Deserialize<'de> for GuildMemberRemoveEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let guild_id = value["d"]["guild_id"]
            .as_str()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| serde::de::Error::custom("Invalid guild_id"))?;

        let user_id = value["d"]["user"]["id"]
            .as_str()
            .and_then(|s| s.parse::<u64>().ok())
            .ok_or_else(|| serde::de::Error::custom("Invalid user_id"))?;

        Ok(GuildMemberRemoveEvent {
            guild_id,
            user_id,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildCreateEvent {
    #[serde(flatten)]
    pub guild: GatewayGuild,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildUpdateEvent {
    #[serde(flatten)]
    pub guild: Guild,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildDeleteEvent {
    #[serde(flatten)]
    pub guild: UnavailableGuild,
}

impl GuildDeleteEvent {
    pub fn is_unavailable(&self) -> bool {
        self.guild.unavailable.is_some()
    }

    pub fn user_left(&self) -> bool {
        self.guild.unavailable.is_none()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildAuditLogEntryCreateEvent {
    #[serde(flatten)]
    pub entry: AuditLogEntry,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}
