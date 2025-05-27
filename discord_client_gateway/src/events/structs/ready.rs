use discord_client_structs::deserializer::deserialize_map_of_u64_string;
use discord_client_structs::structs::channel::Channel;
use discord_client_structs::structs::gateway::GatewayApplication;
use discord_client_structs::structs::guild::clan::GuildJoinRequest;
use discord_client_structs::structs::guild::user::UserGuildSettings;
use discord_client_structs::structs::guild::{GatewayGuild, SupplementalGuild};
use discord_client_structs::structs::misc::{TutorialIndicators, Versioned};
use discord_client_structs::structs::user::connection::Connection;
use discord_client_structs::structs::user::presence::{MergedPresences, Presence};
use discord_client_structs::structs::user::relationship::{GameRelationship, Relationship};
use discord_client_structs::structs::user::session::Session;
use discord_client_structs::structs::user::{Member, User};
use serde::Deserialize;

use discord_client_macros::EnumFromPrimitive;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct ReadyEvent {
    pub session_id: String,
    pub session_type: String,
    pub static_client_session_id: String,
    pub auth_token: Option<String>,
    pub analytics_token: String,
    pub auth_session_id_hash: String,
    pub guilds: Vec<GatewayGuild>,
    pub user: User,
    pub user_guild_settings: Option<Versioned<UserGuildSettings>>,
    pub v: u8,
    pub user_settings_proto: Option<String>,
    pub guild_join_requests: Vec<GuildJoinRequest>,
    pub relationships: Vec<Relationship>,
    pub game_relationships: Vec<GameRelationship>,
    pub friend_suggestion_count: Option<u16>,
    pub private_channels: Vec<Channel>,
    pub connected_accounts: Vec<Connection>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_map_of_u64_string")]
    pub notes: HashMap<u64, String>,
    #[serde(default)]
    pub presences: Vec<Presence>,
    pub merged_presences: Option<MergedPresences>,
    pub merged_members: Option<Vec<Vec<Member>>>,
    #[serde(default)]
    pub users: Option<Vec<User>>,
    pub application: Option<GatewayApplication>,
    pub sessions: Vec<Session>,
    #[serde(default)]
    pub auth: Option<Auth>,
    pub required_action: Option<String>,
    pub country_code: String,
    pub geo_ordered_rtc_regions: Vec<String>,
    pub tutorial: Option<TutorialIndicators>,
    pub shard: Option<Vec<u8>>,
    pub resume_gateway_url: String,
    pub api_code_version: Option<u8>,
    //pub experiments: Vec<UserExperiment>,
    //pub guild_experiments: Vec<GuildExperiment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Auth {
    #[serde(default)]
    pub authenticator_types: Option<Vec<AuthenticatorType>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReadySupplementalEvent {
    pub guilds: Vec<SupplementalGuild>,
    pub merged_members: Vec<Vec<Member>>,
    pub merged_presences: MergedPresences,
    pub lazy_private_channels: Vec<Channel>,
    pub disclose: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResumedEvent {
    #[serde(rename = "_trace")]
    pub trace: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum AuthenticatorType {
    #[default]
    Webauthn = 1,
    Totp = 2,
    Sms = 3,
    Unknown(u8),
}
