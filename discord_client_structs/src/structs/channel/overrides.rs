use crate::structs::channel::deserialize_string_to_u64;
use crate::structs::misc::MuteConfig;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelOverride {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub channel_id: u64,
    pub collapsed: bool,
    pub flags: Option<u64>,
    pub message_notifications: u64,
    pub muted: bool,
    pub mute_config: Option<MuteConfig>,
}
