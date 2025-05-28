use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::channel::Channel;
use crate::structs::guild::role::Role;
use derive_builder::Builder;
use discord_client_macros::Flags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct CreateGuild {
    pub name: String,
    pub description: Option<String>,
    pub region: Option<String>,
    pub icon: Option<String>,
    pub verification_level: Option<u8>,
    pub default_message_notifications: Option<u8>,
    pub explicit_content_filter: Option<u8>,
    pub preferred_locale: Option<String>,
    pub roles: Option<Vec<Role>>,
    pub channels: Option<Vec<Channel>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub system_channel_id: Option<u64>,
    #[serde(default)]
    #[flag_enum(
        "SuppressJoinNotifications=0,SuppressPremiumSubscriptions=1,SuppressGuildReminderNotifications=2,SuppressJoinNotificationReplies=3,SuppressRoleSubscriptionPurchaseNotifications=4,SuppressRoleSubscriptionPurchaseNotificationReplies=5,SuppressChannelPromptDeadchat=7"
    )]
    pub system_channel_flags: Option<u64>,
    pub guild_template_code: Option<String>,
    pub staff_only: bool,
}
