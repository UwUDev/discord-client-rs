use crate::deserializer::deserialize_option_iso8601_string_to_date;
use crate::structs::misc::Emoji;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Poll {
    pub question: PollMedia,
    pub answers: Vec<PollAnswer>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub expiry: Option<DateTime<Utc>>,
    pub allow_multiselect: bool,
    pub layout_type: u8,
    pub results: Option<PollResults>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollMedia {
    pub text: String,
    pub emoji: Option<Emoji>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollAnswer {
    pub answer_id: u64,
    pub poll_media: PollMedia,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollResults {
    pub is_finalized: bool,
    pub answer_counts: Vec<PollAnswerCount>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollAnswerCount {
    pub id: u64,
    pub count: u64,
    pub me_voted: bool,
}
