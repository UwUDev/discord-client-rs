use crate::structs::message::deserialize_string_to_vec_u64;
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

#[derive(Debug, Deserialize, Clone)]
pub struct DebouncedReaction {
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub users: Vec<u64>,
    pub emoji: Emoji,
}