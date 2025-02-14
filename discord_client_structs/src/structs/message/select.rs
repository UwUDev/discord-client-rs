use crate::structs::misc::Emoji;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}
