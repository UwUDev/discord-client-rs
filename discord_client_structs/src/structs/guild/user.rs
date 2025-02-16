// User Guild Settings Structure
//
// Field	Type	Description
// channel_overrides	array[channel override object]	The overrides for channels
// flags	integer	The user guild settings flags
// guild_id 1	?snowflake	The ID of the guild
// hide_muted_channels	boolean	Whether to hide muted channels from the UI
// message_notifications	integer	The message notification level for the guild
// mobile_push	boolean	Whether to send push notifications to mobile clients
// mute_scheduled_events	boolean	Whether new guild scheduled event notifications are muted
// muted	boolean	Whether the guild is muted
// mute_config	?mute config object	The mute metadata for the guild
// notify_highlights	integer	The highlight notification level for the guild
// suppress_everyone	integer	Whether to suppress @everyone notifications
// suppress_roles	integer	Whether to suppress role notifications
// version	integer	The version of guild settings

use crate::structs::guild::deserialize_option_string_to_u64;
use crate::structs::misc::MuteConfig;
use crate::structs::channel::overrides::ChannelOverride;
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