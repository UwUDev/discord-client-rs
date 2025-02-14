use crate::events::structs::channel::thread::{
    ThreadCreateEvent, ThreadDeleteEvent, ThreadListSyncEvent, ThreadUpdateEvent,
};
use crate::events::structs::channel::{ChannelCreateEvent, ChannelDeleteEvent, ChannelUpdateEvent};
use crate::events::structs::gateway::GatewayReconnectEvent;
use crate::events::structs::message::{MessageCreateEvent, MessageReactionAddEvent};
use crate::events::structs::presence::PresenceUpdateEvent;
use crate::events::structs::ready::ReadyEvent;
use serde_json::Value;

pub mod channel;
pub mod gateway;
pub mod message;
pub mod presence;
pub mod ready;

#[derive(Debug, Clone)]
pub enum Event {
    // Ready events
    Ready(ReadyEvent),

    // message events
    MessageCreate(MessageCreateEvent),
    MessageReactionAdd(MessageReactionAddEvent),

    // gateway events
    GatewayReconnect(GatewayReconnectEvent),

    // presence events
    PresenceUpdate(PresenceUpdateEvent),

    // channel events
    ChannelCreate(ChannelCreateEvent),
    ChannelUpdate(ChannelUpdateEvent),
    ChannelDelete(ChannelDeleteEvent),

    // thread events
    ThreadCreate(ThreadCreateEvent),
    ThreadUpdate(ThreadUpdateEvent),
    ThreadDelete(ThreadDeleteEvent),
    ThreadListSync(ThreadListSyncEvent),

    // misc events
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub event_type: String,
    pub data: Value,
    pub op: u8,
}
