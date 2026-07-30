use crate::structs::user::User;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct SoundboardSound {
    #[snowflake]
    pub sound_id: u64,
    pub name: String,
    pub volume: f64,
    #[snowflake]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    pub available: bool,
    pub user: Option<User>,
    #[serde(default)]
    #[snowflake]
    pub user_id: Option<u64>,
}
