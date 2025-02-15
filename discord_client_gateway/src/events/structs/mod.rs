use crate::events::structs::call::*;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::{
    ThreadCreateEvent, ThreadDeleteEvent, ThreadListSyncEvent, ThreadUpdateEvent,
};
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::*;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use serde_json::Value;

pub mod call;
pub mod channel;
pub mod gateway;
pub mod guild;
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
    ConversationSummaryUpdate(ConversationSummaryUpdateEvent),

    // thread events
    ThreadCreate(ThreadCreateEvent),
    ThreadUpdate(ThreadUpdateEvent),
    ThreadDelete(ThreadDeleteEvent),
    ThreadListSync(ThreadListSyncEvent),

    // call events
    CallCreate(CallCreateEvent),

    // voice events
    VoiceStateUpdate(VoiceStateUpdateEvent),
    VoiceChannelStatusUpdate(VoiceChannelStatusUpdateEvent),

    // guild events
    PassiveUpdateV2(PassiveUpdateV2Event),

    // misc events
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub event_type: String,
    pub data: Value,
    pub op: u8,
}
