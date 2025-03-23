use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::channel::Channel;
use crate::structs::channel::voice::VoiceState;
use crate::structs::guild::automod::AutomodIncidentsData;
use crate::structs::guild::clan::Clan;
use crate::structs::guild::event::GuildScheduledEvent;
use crate::structs::guild::role::Role;
use crate::structs::guild::stage::StageInstance;
use crate::structs::message::sticker::Sticker;
use crate::structs::misc::Emoji;
use crate::structs::user::Member;
use crate::structs::user::presence::Presence;
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

pub mod activity;
pub mod automod;
pub mod clan;
pub mod create;
pub mod event;
pub mod experiment;
pub mod integration;
pub mod log;
pub mod role;
pub mod stage;
pub mod user;

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct GatewayGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner: Option<bool>,
    pub permissions: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub widget_channel_id: Option<u64>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    pub roles: Option<Vec<Role>>,
    #[serde(default)]
    pub emojis: Option<Vec<Emoji>>,
    #[serde(default)]
    pub features: Option<Vec<String>>,
    pub mfa_level: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub application_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub system_channel_id: Option<u64>,
    pub system_channel_flags: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub rules_channel_id: Option<u64>,
    pub joined_at: Option<String>,
    pub large: Option<bool>,
    pub unavailable: Option<bool>,
    pub member_count: Option<u32>,
    pub voice_states: Option<Vec<VoiceState>>,
    #[serde(default)]
    pub members: Option<Vec<Member>>,
    pub channels: Option<Vec<Channel>>,
    pub threads: Option<Vec<Channel>>,
    pub presences: Option<Vec<Presence>>,
    pub stage_instances: Option<Vec<StageInstance>>,
    pub guild_scheduled_events: Option<Vec<GuildScheduledEvent>>,
    pub data_mode: Option<String>,
    pub properties: Option<Guild>,
    pub stickers: Option<Vec<Sticker>>,
    pub premium_subscription_count: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct EntityMetadata {
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct Guild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub home_header: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub owner_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub application_id: Option<u64>,
    pub description: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub widget_channel_id: Option<u64>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    #[serde(default)]
    pub features: Option<Vec<String>>,
    #[serde(default)]
    pub roles: Vec<Role>,
    #[serde(default)]
    pub emojis: Option<Vec<Emoji>>,
    #[serde(default)]
    pub stickers: Option<Vec<Sticker>>,
    pub mfa_level: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub system_channel_id: Option<u64>,
    pub system_channel_flags: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub rules_channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub public_updates_channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub safety_alerts_channel_id: Option<u64>,
    pub max_members: Option<u32>,
    pub vanity_url_code: Option<String>,
    pub premium_tier: u8,
    pub premium_subscription_count: Option<u32>,
    pub preferred_locale: String,
    pub max_video_channel_users: Option<u32>,
    pub max_stage_video_channel_users: Option<u32>,
    pub nsfw_level: u8,
    pub hub_type: Option<u8>,
    pub premium_progress_bar_enabled: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub latest_onboarding_question_id: Option<u64>,
    pub incidents_data: Option<AutomodIncidentsData>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub clan: Option<Clan>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct UnavailableGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub unavailable: Option<bool>,
    pub geo_restricted: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct SupplementalGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(default)]
    pub voice_states: Option<Vec<VoiceState>>,
    // todo: "embedded_activities": [],
    // todo: "activity_instances": []
}
