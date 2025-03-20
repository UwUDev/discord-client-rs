use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelRecipientAddEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub user: User,
    pub nick: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelRecipientRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub user: User,
}
