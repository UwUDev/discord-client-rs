use crate::events::structs::call::*;
use crate::events::structs::channel::pin::*;
use crate::events::structs::channel::recipient::*;
use crate::events::structs::channel::stage::*;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::*;
use crate::events::structs::channel::typing::*;
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::webhook::WebhooksUpdateEvent;
use crate::events::structs::channel::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::ack::*;
use crate::events::structs::guild::automod::*;
use crate::events::structs::guild::ban::*;
use crate::events::structs::guild::emoji::*;
use crate::events::structs::guild::integration::*;
use crate::events::structs::guild::join_request::*;
use crate::events::structs::guild::role::*;
use crate::events::structs::guild::schedule_event::*;
use crate::events::structs::guild::soundboard::*;
use crate::events::structs::guild::sticker::*;
use crate::events::structs::guild::unread::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::poll::*;
use crate::events::structs::message::reaction::*;
use crate::events::structs::message::*;
use crate::events::structs::misc::*;
use crate::events::structs::notifications::GenericPushNotificationSentEvent;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use crate::events::structs::requested::*;
use crate::events::structs::user::direct_message::DirectMessageSettingsUpsellShowEvent;
use crate::events::structs::user::note::UserNoteUpdateEvent;
use crate::events::structs::user::relationship::*;
use crate::events::structs::user::*;
use serde_json::Value;

pub mod call;
pub mod channel;
pub mod gateway;
pub mod guild;
pub mod message;
pub mod misc;
pub mod notifications;
pub mod presence;
pub mod ready;
pub mod requested;
pub mod user;

#[derive(Debug, Clone)]
pub enum Event {
    // Ready events
    Ready(ReadyEvent),
    ReadySupplemental(ReadySupplementalEvent),
    Resumed(ResumedEvent),

    // message events
    MessageCreate(MessageCreateEvent),
    MessageUpdate(MessageUpdateEvent),
    MessageDelete(MessageDeleteEvent),
    MessageDeleteBulk(MessageDeleteBulkEvent),
    MessageAck(MessageAckEvent),

    // message reaction events
    MessageReactionAdd(MessageReactionAddEvent),
    MessageReactionRemove(MessageReactionRemoveEvent),
    MessageReactionAddMany(MessageReactionAddManyEvent),
    MessageReactionRemoveEmoji(MessageReactionRemoveEmojiEvent),
    MessageReactionRemoveAll(MessageReactionRemoveAllEvent),

    // gateway events
    GatewayReconnect(GatewayReconnectEvent),
    SessionsReplace(SessionsReplaceEvent),
    HeartbeatAck(HeartbeatAckEvent),
    InvalidSession(InvalidSessionEvent),
    RemoteCommandEvent(RemoteCommandEvent),

    // presence events
    PresenceUpdate(PresenceUpdateEvent),

    // channel events
    ChannelCreate(ChannelCreateEvent),
    ChannelUpdate(ChannelUpdateEvent),
    ChannelDelete(ChannelDeleteEvent),

    // channel summary events
    ConversationSummaryUpdate(ConversationSummaryUpdateEvent),
    ChannelUnreadUpdate(ChannelUnreadUpdateEvent),

    // channel pin events
    ChannelPinsUpdate(ChannelPinsUpdateEvent),
    ChannelPinsAck(ChannelPinsAckEvent),

    // channel recipient events
    ChannelRecipientAdd(ChannelRecipientAddEvent),
    ChannelRecipientRemove(ChannelRecipientRemoveEvent),

    // Channel webhook events
    WebhooksUpdate(WebhooksUpdateEvent),

    // Stages instance events
    StageInstanceCreate(StageInstanceCreateEvent),
    StageInstanceUpdate(StageInstanceUpdateEvent),
    StageInstanceDelete(StageInstanceDeleteEvent),

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
    GuildCreate(GuildCreateEvent),
    GuildUpdate(GuildUpdateEvent),
    GuildDelete(GuildDeleteEvent),
    GuildFeatureAck(GuildFeatureAckEvent),

    // guild member events
    GuildMemberUpdate(GuildMemberUpdateEvent),
    GuildMemberAdd(GuildMemberAddEvent),
    GuildMemberRemove(GuildMemberRemoveEvent),

    // guild log events
    GuildAuditLogEntryCreate(GuildAuditLogEntryCreateEvent),

    // guild role events
    GuildRoleCreate(GuildRoleCreateEvent),
    GuildRoleUpdate(GuildRoleUpdateEvent),
    GuildRoleDelete(GuildRoleDeleteEvent),

    // guild automod events
    AutoModMentionRaidDetection(AutoModMentionRaidDetectionEvent),

    // guild emoji and sticker events
    GuildEmojisUpdate(GuildEmojisUpdateEvent),
    GuildStickersUpdate(GuildStickersUpdateEvent),

    // guild ban events
    GuildBanAdd(GuildBanAddEvent),
    GuildBanRemove(GuildBanRemoveEvent),

    // guild integration events
    GuildIntegrationsUpdate(GuildIntegrationsUpdateEvent),
    IntegrationCreate(IntegrationCreateEvent),
    IntegrationUpdate(IntegrationUpdateEvent),
    IntegrationDelete(IntegrationDeleteEvent),

    // guild schedule events
    GuildScheduledEventCreate(GuildScheduledEventCreateEvent),
    GuildScheduledEventUpdate(GuildScheduledEventUpdateEvent),
    GuildScheduledEventDelete(GuildScheduledEventDeleteEvent),
    GuildScheduledEventExceptionCreate(GuildScheduledEventExceptionCreateEvent),
    GuildScheduledEventExceptionUpdate(GuildScheduledEventExceptionUpdateEvent),
    GuildScheduledEventExceptionDelete(GuildScheduledEventExceptionDeleteEvent),
    GuildScheduledEventExceptionsDelete(GuildScheduledEventExceptionsDeleteEvent),
    GuildScheduledEventUserAdd(GuildScheduledEventUserAddEvent),
    GuildScheduledEventUserRemove(GuildScheduledEventUserRemoveEvent),

    // guild soundboard events
    GuildSoundboardSoundCreate(GuildSoundboardSoundCreateEvent),
    GuildSoundboardSoundUpdate(GuildSoundboardSoundUpdateEvent),
    GuildSoundboardSoundDelete(GuildSoundboardSoundDeleteEvent),

    // guild join requests
    GuildJoinRequestCreate(GuildJoinRequestCreateEvent),
    GuildJoinRequestUpdate(GuildJoinRequestUpdateEvent),
    GuildJoinRequestDelete(GuildJoinRequestDeleteEvent),

    // typing events
    TypingStart(TypingStartEvent),

    // poll events
    MessagePollVoteAdd(MessagePollVoteAddEvent),
    MessagePollVoteRemove(MessagePollVoteRemoveEvent),

    // user events
    UserConnectionsUpdate(UserConnectionsUpdateEvent),
    UserGuildSettingsUpdate(UserGuildSettingsUpdateEvent),
    UserNoteUpdateEvent(UserNoteUpdateEvent),

    // relationship events
    RelationshipAdd(RelationshipAddEvent),
    RelationshipUpdate(RelationshipUpdateEvent),
    RelationshipRemove(RelationshipRemoveEvent),

    // requested events
    ChannelStatuses(ChannelStatusesEvent),
    GuildMembersChunk(GuildMembersChunkEvent),
    LastMessages(LastMessagesEvent),
    SoundboardSounds(SoundboardSoundsEvent),

    //DMs
    DirectMessageSettingsUpsellShow(DirectMessageSettingsUpsellShowEvent),

    // misc events
    GenericPushNotificationSent(GenericPushNotificationSentEvent),
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
