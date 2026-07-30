use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Overwrite {
    #[snowflake]
    pub id: u64,
    pub r#type: u8,
    pub allow: String,
    pub deny: String,
}
