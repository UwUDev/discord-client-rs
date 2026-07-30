use discord_client_macros::discord_struct;

#[discord_struct]
pub struct StageInstance {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub guild_id: u64,
    #[serde(default)]
    #[snowflake]
    pub channel_id: Option<u64>,
    pub topic: String,
    pub privacy_level: u8,
    pub invite_code: Option<String>,
    pub discoverable_disabled: Option<bool>,
    #[snowflake]
    pub guild_scheduled_event_id: Option<u64>,
}
