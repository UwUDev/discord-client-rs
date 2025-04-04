use discord_client_structs::deserializer::*;
use discord_client_structs::structs::channel::unread::ChannelUnreadUpdate;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelUnreadUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub channel_unread_updates: Vec<ChannelUnreadUpdate>,
}
