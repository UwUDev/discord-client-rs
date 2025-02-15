use crate::events::structs::call::*;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::{
    ThreadCreateEvent, ThreadDeleteEvent, ThreadListSyncEvent, ThreadUpdateEvent,
};
use crate::events::structs::channel::typing::*;
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::poll::*;
use crate::events::structs::message::*;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use serde_json::Value;
use crate::events::structs::message::reaction::*;

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
    MessageReactionRemove(MessageReactionRemoveEvent),
    MessageReactionAddMany(MessageReactionAddManyEvent),

    // gateway events
    GatewayReconnect(GatewayReconnectEvent),
    SessionsReplace(SessionsReplaceEvent),
    HeartbeatAck(HeartbeatAckEvent),

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
    GuildMemberUpdate(GuildMemberUpdateEvent),
    GuildMemberAdd(GuildMemberAddEvent),
    GuildMemberRemove(GuildMemberRemoveEvent),

    // typing events
    TypingStart(TypingStartEvent),

    // poll events
    MessagePollVoteAdd(MessagePollVoteAddEvent),
    MessagePollVoteRemove(MessagePollVoteRemoveEvent),

    // misc events
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub event_type: String,
    pub data: Value,
    pub op: u8,
}
