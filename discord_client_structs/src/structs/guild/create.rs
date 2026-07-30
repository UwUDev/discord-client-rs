use crate::structs::channel::Channel;
use crate::structs::guild::role::Role;
use discord_client_macros::discord_struct;

#[discord_struct]
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
    #[snowflake]
    pub afk_channel_id: Option<u64>,
    pub afk_timeout: Option<u32>,
    #[serde(default)]
    #[snowflake]
    pub system_channel_id: Option<u64>,
    #[serde(default)]
    #[flag_enum(
        "SuppressJoinNotifications=0,SuppressPremiumSubscriptions=1,SuppressGuildReminderNotifications=2,SuppressJoinNotificationReplies=3,SuppressRoleSubscriptionPurchaseNotifications=4,SuppressRoleSubscriptionPurchaseNotificationReplies=5,SuppressChannelPromptDeadchat=7"
    )]
    pub system_channel_flags: Option<u64>,
    pub guild_template_code: Option<String>,
    pub staff_only: bool,
}
