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
use crate::events::structs::message::mention::RecentMentionDeleteEvent;
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

    // message mention events
    RecentMentionDelete(RecentMentionDeleteEvent),

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

    // guild boosts
    GuildAppliedBoostsUpdate(GuildAppliedBoostsUpdateEvent),

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
    AuthSessionChange(AuthSessionChangeEvent),
    Unknown(UnknownEvent),
}

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub r#type: String,
    pub data: Value,
    pub op: u8,
}

/// A dispatch event whose type is known and registered, but whose payload
/// failed to deserialize into its struct. Unlike [`UnknownEvent`] (an event
/// type we don't handle at all), this carries the parse error *and* the raw
/// JSON, so a single malformed/outdated event no longer aborts the stream and
/// the offending payload can be dumped and inspected to fix the struct.
#[derive(Debug, Clone)]
pub struct ParseErrorEvent {
    /// The dispatch event type, e.g. `"MESSAGE_CREATE"`.
    pub event_type: String,
    /// The gateway opcode the payload arrived on.
    pub op: u8,
    /// The deserializer error message.
    pub error: String,
    /// The JSON path to the field that failed, from `serde_path_to_error`.
    pub path: String,
    /// The raw, untouched event data (`payload.d`).
    pub raw: Value,
}

impl ParseErrorEvent {
    /// Write the raw payload to `<dir>/<EVENT_TYPE>-<nanos>.json` (pretty-printed),
    /// creating `dir` if needed, and return the path written. Each capture gets its
    /// own file so failures never clobber one another — unlike a single rolling
    /// dump file, which is overwritten by the next frame before you can inspect it.
    pub fn dump_to<P: AsRef<std::path::Path>>(&self, dir: P) -> std::io::Result<std::path::PathBuf> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = dir.join(format!("{}-{}.json", self.event_type, nanos));
        let json =
            serde_json::to_string_pretty(&self.raw).unwrap_or_else(|_| self.raw.to_string());
        std::fs::write(&path, json)?;
        Ok(path)
    }
}
