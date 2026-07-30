use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::misc::Emoji;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Poll {
    pub question: PollMedia,
    pub answers: Vec<PollAnswer>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub expiry: Option<DateTime<Utc>>,
    pub allow_multiselect: bool,
    pub layout_type: u8,
    pub results: Option<PollResults>,
}

#[discord_struct]
pub struct PollMedia {
    pub text: String,
    pub emoji: Option<Emoji>,
}

#[discord_struct]
pub struct PollAnswer {
    pub answer_id: u64,
    pub poll_media: PollMedia,
}

#[discord_struct]
pub struct PollResults {
    pub is_finalized: bool,
    pub answer_counts: Vec<PollAnswerCount>,
}

#[discord_struct]
pub struct PollAnswerCount {
    pub id: u64,
    pub count: u64,
    pub me_voted: bool,
}
