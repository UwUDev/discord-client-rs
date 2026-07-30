use discord_client_macros::discord_struct;

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct ChannelStatus {
    #[snowflake]
    pub id: u64,
    pub status: String,
}
