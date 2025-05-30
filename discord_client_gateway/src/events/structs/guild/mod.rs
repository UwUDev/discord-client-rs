use chrono::{DateTime, Utc};
use discord_client_macros::Flags;
use discord_client_structs::deserializer::*;
use discord_client_structs::structs::channel::UpdatedChannel;
use discord_client_structs::structs::channel::voice::VoiceState;
use discord_client_structs::structs::guild::log::AuditLogEntry;
use discord_client_structs::structs::guild::user::UserGuildSettings;
use discord_client_structs::structs::guild::{GatewayGuild, Guild, UnavailableGuild};
use discord_client_structs::structs::user::{AvatarDecorationData, Member, User};
use serde::{Deserialize, Deserializer};
use serde_json::Value;

pub mod ack;
pub mod automod;
pub mod ban;
pub mod emoji;
pub mod integration;
pub mod join_request;
pub mod role;
pub mod schedule_event;
pub mod soundboard;
pub mod sticker;
pub mod unread;

#[derive(Debug, Deserialize, Clone)]
pub struct PassiveUpdateV2Event {
    pub updated_voice_states: Vec<VoiceState>,
    pub updated_members: Vec<Member>,
    pub updated_channels: Vec<UpdatedChannel>,
    pub removed_voice_states: Vec<Value>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
}

#[derive(Debug, Deserialize, Clone, Flags)]
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
    #[flag_enum(
        "DidRejoin=0,CompletedOnboarding=1,BypassesVerification=2,StartedOnboarding=3,IsGuest=4,StartedServerGuide=5,CompletedServerGuide=6,AutomodQuarantinedName=7,DmSettingsUpsellAcknowledged=9,AutomodQuarantinedClanTag=10"
    )]
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
    pub user: Option<User>,
}

impl<'de> Deserialize<'de> for GuildMemberRemoveEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        let guild_id = value["guild_id"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| serde::de::Error::custom("Format guild_id invalide"))?;

        let user_id = value["user"]["id"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| serde::de::Error::custom("Format user.id invalide"))?;

        let user = match User::deserialize(value["user"].clone()) {
            Ok(u) => Some(u),
            Err(_) => None,
        };

        Ok(GuildMemberRemoveEvent {
            guild_id,
            user_id,
            user,
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

#[derive(Debug, Deserialize, Clone)]
pub struct UserGuildSettingsUpdateEvent {
    #[serde(flatten)]
    pub user_guild_settings: UserGuildSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildAppliedBoostsUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub pause_ends_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub ends_at: Option<DateTime<Utc>>,
    pub ended: bool,
}
