use crate::structs::user::User;
use crate::structs::user::connection::IntegrationGuild;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Webhook {
    #[snowflake]
    pub id: u64,
    pub type_: u8,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    #[snowflake]
    pub channel_id: u64,
    #[serde(default)]
    pub user: Option<User>,
    pub name: Option<String>,
    pub avatar: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
    #[snowflake]
    pub application_id: u64,
    #[serde(default)]
    pub source_guild: Option<IntegrationGuild>,
    #[serde(default)]
    pub source_channel: Option<WebhookChannel>,
    #[serde(default)]
    pub url: Option<String>,
}

#[discord_struct]
pub struct WebhookChannel {
    #[snowflake]
    pub id: u64,
    pub name: String,
}
