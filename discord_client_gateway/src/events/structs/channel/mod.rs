use discord_client_structs::structs::channel::Channel;
use serde::Deserialize;

pub mod thread;

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelCreateEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelUpdateEvent {
    #[serde(flatten)]
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelDeleteEvent {
    #[serde(flatten)]
    pub channel: Channel,
}
