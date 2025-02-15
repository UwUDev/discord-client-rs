use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::summary::Summary;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ConversationSummaryUpdateEvent {
    pub summaries: Vec<Summary>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
}
