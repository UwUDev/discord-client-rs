use crate::structs::misc::Emoji;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Reaction {
    pub count: u64,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReactionCountDetails {
    pub normal: u64,
    pub burst: u64,
}
