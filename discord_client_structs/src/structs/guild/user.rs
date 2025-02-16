use crate::structs::channel::overrides::ChannelOverride;
use crate::structs::guild::deserialize_option_string_to_u64;
use crate::structs::misc::MuteConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UserGuildSettings {
    pub channel_overrides: Vec<ChannelOverride>,
    pub flags: u64, // todo: https://docs.discord.sex/resources/user-settings#user-guild-settings-flags
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub hide_muted_channels: bool,
    pub message_notifications: u64,
    pub mobile_push: bool,
    pub mute_scheduled_events: bool,
    pub muted: bool,
    pub mute_config: Option<MuteConfig>,
    pub notify_highlights: u64,
    pub suppress_everyone: bool,
    pub suppress_roles: bool,
    pub version: u64,
}
