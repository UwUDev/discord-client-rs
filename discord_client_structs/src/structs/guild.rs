use crate::deserializer::deserialize_option_iso8601_string_to_date;
use crate::deserializer::deserialize_string_to_u64;
use crate::deserializer::deserialize_option_string_to_u64;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use crate::structs::channel::Channel;
use crate::structs::channel::voice::VoiceState;
use crate::structs::message::sticker::Sticker;
use crate::structs::misc::Emoji;
use crate::structs::user::Member;
use crate::structs::user::presence::Presence;

// todo: refactor

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub icon: Option<String>,
    pub owner: Option<bool>,
    pub permissions: Option<String>,
    #[serde(default)]
    pub region: Option<String>,
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
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
    pub application_id: Option<u64>,
    pub system_channel_id: Option<u64>,
    pub system_channel_flags: Option<u8>,
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

#[derive(Debug, Deserialize, Clone)]
pub struct StageInstance {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    pub topic: String,
    pub privacy_level: u8,
    pub invite_code: Option<String>,
    pub discoverable_disabled: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_scheduled_event_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildScheduledEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub creator_id: Option<u64>,
    pub creator: Option<Member>,
    #[serde(default)]
    pub name: Option<String>,
    pub description: Option<String>,
    pub scheduled_start_time: String,
    pub scheduled_end_time: Option<String>,
    pub privacy_level: u8,
    pub status: u8,
    pub entity_type: u8,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub entity_id: Option<u64>,
    pub entity_metadata: Option<EntityMetadata>,
    pub user_count: Option<u32>,
    pub image: Option<String>,
}
#[derive(Debug, Deserialize, Clone)]
pub struct EntityMetadata {
    pub location: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: u32,
    pub hoist: bool,
    pub icon: Option<String>,
    pub unicode_emoji: Option<String>,
    pub position: i32,
    pub permissions: String,
    pub managed: bool,
    pub mentionable: bool,
    pub flags: Option<u32>,
    pub tags: Option<RoleTags>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RoleTags {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub bot_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub integration_id: Option<u64>,
    pub premium_subscriber: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub subscription_listing_id: Option<u64>,
    pub available_for_purchase: Option<bool>,
    pub guild_connections: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Guild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub home_header: Option<String>,
    pub splash: Option<String>,
    pub discovery_splash: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub owner_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub description: Option<String>,
    pub region: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    pub widget_enabled: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
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
    pub system_channel_id: Option<u64>,
    pub system_channel_flags: Option<u8>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub rules_channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub public_updates_channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
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
    pub latest_onboarding_question_id: Option<u64>,
    pub incidents_data: Option<AutomodIncidentsData>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    pub clan: Option<Clan>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AutomodIncidentsData {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub raid_detected_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub dm_spam_detected_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub invites_disabled_until: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub dms_disabled_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Clan {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    pub name: Option<String>,
    pub tag: String,
    pub icon_hash: Option<String>,
    pub member_count: u32,
    pub description: Option<String>,
    pub play_style: Option<u8>,
    pub search_terms: Option<Vec<String>>,
    pub game_application_ids: Option<Vec<u64>>,
    pub badge: Option<u8>,
    pub badge_hash: Option<String>,
    pub badge_color_primary: Option<String>,
    pub badge_color_secondary: Option<String>,
    pub banner: Option<u8>,
    pub banner_hash: Option<String>,
    pub brand_color_primary: Option<String>,
    pub brand_color_secondary: Option<String>,
    pub wildcard_descriptors: Option<Vec<String>>,
    pub game_activity: Option<Vec<GameActivity>>,
    pub discovery_profile_features: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameActivity {
    pub activity_level: u32,
    pub activity_score: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UnavailableGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub unavailable: Option<bool>,
    pub geo_restricted: Option<bool>,
}