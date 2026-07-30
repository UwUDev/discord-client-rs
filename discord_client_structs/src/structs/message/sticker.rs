use crate::structs::user::User;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct StickerItem {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub format_type: u8,
}

#[discord_struct]
pub struct Sticker {
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    #[snowflake]
    pub pack_id: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub sticker_type: Option<u8>,
    pub format_type: u8,
    pub available: Option<bool>,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    pub user: Option<User>,
    pub sort_value: Option<u64>,
}
