use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Role {
    #[snowflake]
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
    #[flag_enum("InPrompt=0")]
    pub flags: Option<u64>,
    pub tags: Option<RoleTags>,
}

#[discord_struct]
pub struct RoleTags {
    #[serde(default)]
    #[snowflake]
    pub bot_id: Option<u64>,
    #[serde(default)]
    #[snowflake]
    pub integration_id: Option<u64>,
    pub premium_subscriber: Option<bool>,
    #[serde(default)]
    #[snowflake]
    pub subscription_listing_id: Option<u64>,
    pub available_for_purchase: Option<bool>,
    pub guild_connections: Option<bool>,
}
