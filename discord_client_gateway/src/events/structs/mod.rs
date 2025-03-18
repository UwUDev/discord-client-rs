use crate::events::structs::call::*;
use crate::events::structs::channel::pin::*;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::*;
use crate::events::structs::channel::typing::*;
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::role::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::poll::*;
use crate::events::structs::message::reaction::*;
use crate::events::structs::message::*;
use crate::events::structs::misc::*;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use crate::events::structs::user::*;
use serde_json::Value;

pub mod call;
pub mod channel;
pub mod gateway;
pub mod guild;
pub mod message;
pub mod misc;
pub mod presence;
pub mod ready;
pub mod user;

#[derive(Debug, Clone)]
pub enum Event {
    // Ready events
    Ready(ReadyEvent),
    ReadySupplemental(ReadySupplementalEvent),

    // message events
    MessageCreate(MessageCreateEvent),
    MessageUpdate(MessageUpdateEvent),
    MessageDelete(MessageDeleteEvent),
    MessageDeleteBulk(MessageDeleteBulkEvent),
    MessageAck(MessageAckEvent),
    MessageReactionAdd(MessageReactionAddEvent),
    MessageReactionRemove(MessageReactionRemoveEvent),
    MessageReactionAddMany(MessageReactionAddManyEvent),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiEvent),
    MessageReactionRemoveAll(MessageReactionRemoveAllEvent),

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
    ChannelPinsUpdate(ChannelPinsUpdateEvent),
    ChannelPinsAck(ChannelPinsAckEvent),

    // thread events
    ThreadCreate(ThreadCreateEvent),
    ThreadUpdate(ThreadUpdateEvent),
    ThreadDelete(ThreadDeleteEvent),
    ThreadListSync(ThreadListSyncEvent),
    ThreadMemberUpdate(ThreadMemberUpdateEvent),
    ThreadMembersUpdate(ThreadMembersUpdateEvent),

    // call events
    CallCreate(CallCreateEvent),
    CallUpdate(CallUpdateEvent),
    CallDelete(CallDeleteEvent),

    // voice events
    VoiceStateUpdate(VoiceStateUpdateEvent),
    VoiceChannelStatusUpdate(VoiceChannelStatusUpdateEvent),

    // guild events
    PassiveUpdateV2(PassiveUpdateV2Event),
    GuildMemberUpdate(GuildMemberUpdateEvent),
    GuildMemberAdd(GuildMemberAddEvent),
    GuildMemberRemove(GuildMemberRemoveEvent),
    GuildCreate(GuildCreateEvent),
    GuildUpdate(GuildUpdateEvent),
    GuildDelete(GuildDeleteEvent),
    GuildAuditLogEntryCreate(GuildAuditLogEntryCreateEvent),
    GuildRoleCreate(GuildRoleCreateEvent),
    GuildRoleUpdate(GuildRoleUpdateEvent),
    GuildRoleDelete(GuildRoleDeleteEvent),

    // typing events
    TypingStart(TypingStartEvent),

    // poll events
    MessagePollVoteAdd(MessagePollVoteAddEvent),
    MessagePollVoteRemove(MessagePollVoteRemoveEvent),

    // user events
    UserConnectionsUpdate(UserConnectionsUpdateEvent),

    // misc events
    ContentInventoryInboxStale(ContentInventoryInboxStaleEvent),
    UserSettingsProtoUpdate(UserSettingsProtoUpdateEvent),
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub r#type: String,
    pub data: Value,
    pub op: u8,
}
