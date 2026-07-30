use crate::structs::application::ApplicationCommand;
use crate::structs::channel::Channel;
use crate::structs::channel::webhook::Webhook;
use crate::structs::guild::automod::AutomodRule;
use crate::structs::guild::event::GuildScheduledEvent;
use crate::structs::user::User;
use crate::structs::user::connection::PartialIntegration;
use discord_client_macros::discord_struct;

pub mod query;

#[discord_struct]
pub struct AuditLog {
    pub audit_log_entries: Vec<AuditLogEntry>,
    pub application_commands: Vec<ApplicationCommand>,
    pub auto_moderation_rules: Vec<AutomodRule>,
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,
    pub integrations: Vec<PartialIntegration>,
    pub threads: Vec<Channel>,
    pub users: Vec<User>,
    pub webhooks: Vec<Webhook>,
}

#[discord_struct]
pub struct AuditLogEntry {
    #[serde(default)]
    #[snowflake]
    pub target_id: Option<u64>,
    #[serde(default)]
    pub changes: Option<Vec<AuditLogChange>>,
    #[serde(default)]
    #[snowflake]
    pub user_id: Option<u64>,
    #[snowflake]
    pub id: u64,
    pub action_type: u8,
    #[serde(default)]
    pub options: Option<AuditEntryInfo>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[discord_struct]
pub struct AuditLogChange {
    #[serde(default)]
    // can be an object, array, string, number and even boolean 💀
    pub new_value: Option<serde_json::Value>,
    #[serde(default)]
    pub old_value: Option<serde_json::Value>,
    pub key: String,
}
#[discord_struct]
pub struct AuditEntryInfo {
    #[serde(default)]
    #[snowflake]
    pub application_id: Option<u64>,
    #[serde(default)]
    pub auto_moderation_rule_name: Option<String>,
    #[serde(default)]
    pub auto_moderation_rule_trigger_type: Option<String>,
    #[serde(default)]
    #[snowflake]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub count: Option<String>,
    #[serde(default)]
    pub delete_member_days: Option<String>,
    #[serde(default)]
    #[snowflake]
    pub id: Option<u64>,
    #[serde(default)]
    pub integration_type: Option<String>,
    #[serde(default)]
    pub members_removed: Option<String>,
    #[serde(default)]
    #[snowflake]
    pub message_id: Option<u64>,
    #[serde(default)]
    pub role_name: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    pub status: Option<String>,
}
