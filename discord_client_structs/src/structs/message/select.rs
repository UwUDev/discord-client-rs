use crate::structs::misc::Emoji;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}
