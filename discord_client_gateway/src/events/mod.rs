use crate::events::gateway::GatewayPayload;
use crate::events::structs::call::*;
use crate::events::structs::channel::pin::*;
use crate::events::structs::channel::recipient::*;
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
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use crate::events::structs::requested::*;
use crate::events::structs::user::*;
use crate::events::structs::*;

pub(crate) mod deserializer;
pub mod structs;

macro_rules! define_events {
    (
        dispatch op $dispatch_op:expr, {
            $( $variant:ident { t: $t:expr, type: $event_struct:ty } ),+ $(,)?
        }
        $(
            , non_dispatch op $nd_op:expr, {
                $( $nd_variant:ident { type: $nd_struct:ty } ),+ $(,)?
            }
        )*
    ) => {
        #[derive(Debug, Clone)]
        pub enum Event {
            // Dispatch events
            $(
                $variant($event_struct),
            )+

            // Non-dispatch events
            $(
                $(
                    $nd_variant($nd_struct),
                )+
            )*

            Unknown(UnknownEvent),
        }

        impl Event {
            pub fn event_name(&self) -> &str {
                match self {
                    $(
                        Event::$variant(_) => $t,
                    )+
                    $(
                        $(
                            Event::$nd_variant(_) => stringify!($nd_variant),
                        )+
                    )*
                    Event::Unknown(unknown) => &unknown.r#type,
                }
            }
        }

        impl std::fmt::Display for Event {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Event::$variant(_) => write!(f, "{}", $t),
                    )+
                    $(
                        $(
                            Event::$nd_variant(_) => write!(f, "{}", stringify!($nd_variant)),
                        )+
                    )*
                    Event::Unknown(unknown) => write!(f, "Unknown ({}): {}", unknown.op, unknown.r#type),
                }
            }
        }

        pub fn parse_gateway_payload(payload: GatewayPayload) -> Result<Event, serde_json::Error> {
            match payload.op {
                // Dispatch events
                $dispatch_op => match payload.t.as_deref() {
                    $(
                        Some($t) => {
                            let json_string = serde_json::to_string(&payload.d)
                                .map_err(|e| serde::de::Error::custom(format!("Unable to process JSON conversion: {}", e)))?;

                            let jd = &mut serde_json::Deserializer::from_str(&json_string);
                            let result: Result<$event_struct, _> = serde_path_to_error::deserialize(jd);

                            match result {
                                Ok(event) => Ok(Event::$variant(event)),
                                Err(err) => {
                                    let chemin = err.path().to_string();
                                    let message = format!("Unable to parse event {}: {} at '{}'",
                                                         $t, err.inner(), chemin);
                                    Err(serde::de::Error::custom(message))
                                }
                            }
                        },
                    )+
                    Some(other) => Ok(Event::Unknown(UnknownEvent {
                        r#type: other.to_string(),
                        data: payload.d,
                        op: payload.op,
                    })),
                    None => Err(serde::de::Error::custom("Dispatch event missing 't' field")),
                },

                // Non-dispatch events
                $(
                    $nd_op => match serde_json::from_value(payload.d.clone()) {
                        $(
                            Ok(data) => Ok(Event::$nd_variant(data)),
                        )+
                        Err(_) => Ok(Event::Unknown(UnknownEvent {
                            r#type: "UNKNOWN_NON_DISPATCH".to_string(),
                            data: payload.d,
                            op: payload.op,
                        })),
                    },
                )*

                // Unknown opcodes
                _ => Ok(Event::Unknown(UnknownEvent {
                    r#type: "UNKNOWN_OP".to_string(),
                    data: payload.d,
                    op: payload.op,
                })),
            }
        }
    };
}

define_events! {
    dispatch op 0, {
        AutoModMentionRaidDetection { t: "AUTO_MODERATION_MENTION_RAID_DETECTION", type: AutoModMentionRaidDetectionEvent },
        CallCreate { t: "CALL_CREATE", type: CallCreateEvent },
        CallDelete { t: "CALL_DELETE", type: CallDeleteEvent },
        CallUpdate { t: "CALL_UPDATE", type: CallUpdateEvent },
        ChannelCreate { t: "CHANNEL_CREATE", type: ChannelCreateEvent },
        ChannelDelete { t: "CHANNEL_DELETE", type: ChannelDeleteEvent },
        ChannelPinsAck { t: "CHANNEL_PINS_ACK", type: ChannelPinsAckEvent },
        ChannelPinsUpdate { t: "CHANNEL_PINS_UPDATE", type: ChannelPinsUpdateEvent },
        ChannelRecipientAdd { t: "CHANNEL_RECIPIENT_ADD", type: ChannelRecipientAddEvent },
        ChannelRecipientRemove { t: "CHANNEL_RECIPIENT_REMOVE", type: ChannelRecipientRemoveEvent },
        ChannelStatues { t: "CHANNEL_STATUSES", type: ChannelStatusesEvent },
        ChannelUnreadUpdate { t: "CHANNEL_UNREAD_UPDATE", type: ChannelUnreadUpdateEvent },
        ChannelUpdate { t: "CHANNEL_UPDATE", type: ChannelUpdateEvent },
        ContentInventoryInboxStale { t: "CONTENT_INVENTORY_INBOX_STALE", type: ContentInventoryInboxStaleEvent },
        ConversationSummaryUpdate { t: "CONVERSATION_SUMMARY_UPDATE", type: ConversationSummaryUpdateEvent },
        GuildAuditLogEntryCreate { t: "GUILD_AUDIT_LOG_ENTRY_CREATE", type: GuildAuditLogEntryCreateEvent },
        GuildBanAdd { t: "GUILD_BAN_ADD", type: GuildBanAddEvent },
        GuildBanRemove { t: "GUILD_BAN_REMOVE", type: GuildBanRemoveEvent },
        GuildCreate { t: "GUILD_CREATE", type: GuildCreateEvent },
        GuildDelete { t: "GUILD_DELETE", type: GuildDeleteEvent },
        GuildEmojisUpdate { t: "GUILD_EMOJIS_UPDATE", type: GuildEmojisUpdateEvent },
        GuildFeatureAck { t: "GUILD_FEATURE_ACK", type: GuildFeatureAckEvent },
        GuildIntegrationCreate { t: "INTEGRATION_CREATE", type: IntegrationCreateEvent },
        GuildIntegrationDelete { t: "INTEGRATION_DELETE", type: IntegrationDeleteEvent },
        GuildIntegrationsUpdate { t: "GUILD_INTEGRATIONS_UPDATE", type: GuildIntegrationsUpdateEvent },
        GuildIntegrationUpdate { t: "INTEGRATION_UPDATE", type: IntegrationUpdateEvent },
        GuildJoinRequestCreate { t: "GUILD_JOIN_REQUEST_CREATE", type: GuildJoinRequestCreateEvent },
        GuildJoinRequestDelete { t: "GUILD_JOIN_REQUEST_DELETE", type: GuildJoinRequestDeleteEvent },
        GuildJoinRequestUpdate { t: "GUILD_JOIN_REQUEST_UPDATE", type: GuildJoinRequestUpdateEvent },
        GuildMemberAdd { t: "GUILD_MEMBER_ADD", type: GuildMemberAddEvent },
        GuildMemberRemove { t: "GUILD_MEMBER_REMOVE", type: GuildMemberRemoveEvent },
        GuildMembersChunk { t: "GUILD_MEMBERS_CHUNK", type: GuildMembersChunkEvent },
        GuildMemberUpdate { t: "GUILD_MEMBER_UPDATE", type: GuildMemberUpdateEvent },
        GuildRoleCreate { t: "GUILD_ROLE_CREATE", type: GuildRoleCreateEvent },
        GuildRoleDelete { t: "GUILD_ROLE_DELETE", type: GuildRoleDeleteEvent },
        GuildRoleUpdate { t: "GUILD_ROLE_UPDATE", type: GuildRoleUpdateEvent },
        GuildScheduledEventCreate { t: "GUILD_SCHEDULED_EVENT_CREATE", type: GuildScheduledEventCreateEvent },
        GuildScheduledEventDelete { t: "GUILD_SCHEDULED_EVENT_DELETE", type: GuildScheduledEventDeleteEvent },
        GuildScheduledEventExceptionCreate { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_CREATE", type: GuildScheduledEventExceptionCreateEvent },
        GuildScheduledEventExceptionDelete { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_DELETE", type: GuildScheduledEventExceptionDeleteEvent },
        GuildScheduledEventExceptionsDelete { t: "GUILD_SCHEDULED_EVENT_EXCEPTIONS_DELETE", type: GuildScheduledEventExceptionsDeleteEvent },
        GuildScheduledEventExceptionUpdate { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_UPDATE", type: GuildScheduledEventExceptionUpdateEvent },
        GuildScheduledEventUpdate { t: "GUILD_SCHEDULED_EVENT_UPDATE", type: GuildScheduledEventUpdateEvent },
        GuildScheduledEventUserAdd { t: "GUILD_SCHEDULED_EVENT_USER_ADD", type: GuildScheduledEventUserAddEvent },
        GuildScheduledEventUserRemove { t: "GUILD_SCHEDULED_EVENT_USER_REMOVE", type: GuildScheduledEventUserRemoveEvent },
        GuildSoundboardSoundCreate { t: "GUILD_SOUNDBOARD_SOUND_CREATE", type: GuildSoundboardSoundCreateEvent },
        GuildSoundboardSoundDelete { t: "GUILD_SOUNDBOARD_SOUND_DELETE", type: GuildSoundboardSoundDeleteEvent },
        GuildSoundboardSoundUpdate { t: "GUILD_SOUNDBOARD_SOUND_UPDATE", type: GuildSoundboardSoundUpdateEvent },
        GuildStickersUpdate { t: "GUILD_STICKERS_UPDATE", type: GuildStickersUpdateEvent },
        GuildUpdate { t: "GUILD_UPDATE", type: GuildUpdateEvent },
        LastMessages { t: "LAST_MESSAGES", type: LastMessagesEvent },
        MessageAck { t: "MESSAGE_ACK", type: MessageAckEvent },
        MessageCreate { t: "MESSAGE_CREATE", type: MessageCreateEvent },
        MessageDelete { t: "MESSAGE_DELETE", type: MessageDeleteEvent },
        MessageDeleteBulk { t: "MESSAGE_DELETE_BULK", type: MessageDeleteBulkEvent },
        MessagePollVoteAdd { t: "MESSAGE_POLL_VOTE_ADD", type: MessagePollVoteAddEvent },
        MessagePollVoteRemove { t: "MESSAGE_POLL_VOTE_REMOVE", type: MessagePollVoteRemoveEvent },
        MessageReactionAdd { t: "MESSAGE_REACTION_ADD", type: MessageReactionAddEvent },
        MessageReactionAddMany { t: "MESSAGE_REACTION_ADD_MANY", type: MessageReactionAddManyEvent },
        MessageReactionRemove { t: "MESSAGE_REACTION_REMOVE", type: MessageReactionRemoveEvent },
        MessageReactionRemoveAll { t: "MESSAGE_REACTION_REMOVE_ALL", type: MessageReactionRemoveAllEvent },
        MessageReactionRemoveEmoji { t: "MESSAGE_REACTION_REMOVE_EMOJI", type: MessageReactionRemoveEmojiEvent },
        MessageUpdate { t: "MESSAGE_UPDATE", type: MessageUpdateEvent },
        PassiveUpdateV2 { t: "PASSIVE_UPDATE_V2", type: PassiveUpdateV2Event },
        PresenceUpdate { t: "PRESENCE_UPDATE", type: PresenceUpdateEvent },
        Ready { t: "READY", type: ReadyEvent },
        ReadySupplemental { t: "READY_SUPPLEMENTAL", type: ReadySupplementalEvent },
        Resumed { t: "RESUMED", type: ResumedEvent },
        SessionReplace { t: "SESSIONS_REPLACE", type: SessionsReplaceEvent },
        SoundboardSounds { t: "SOUNDBOARD_SOUNDS", type: SoundboardSoundsEvent },
        ThreadCreate { t: "THREAD_CREATE", type: ThreadCreateEvent },
        ThreadDelete { t: "THREAD_DELETE", type: ThreadDeleteEvent },
        ThreadListSync { t: "THREAD_LIST_SYNC", type: ThreadListSyncEvent },
        ThreadMembersUpdate { t: "THREAD_MEMBERS_UPDATE", type: ThreadMembersUpdateEvent },
        ThreadMemberUpdate { t: "THREAD_MEMBER_UPDATE", type: ThreadMemberUpdateEvent },
        ThreadUpdate { t: "THREAD_UPDATE", type: ThreadUpdateEvent },
        TypingStart { t: "TYPING_START", type: TypingStartEvent },
        UserConnectionsUpdate { t: "USER_CONNECTIONS_UPDATE", type: UserConnectionsUpdateEvent },
        UserSettingsProtoUpdate { t: "USER_SETTINGS_PROTO_UPDATE", type: UserSettingsProtoUpdateEvent },
        VoiceChannelStatusUpdate { t: "VOICE_CHANNEL_STATUS_UPDATE", type: VoiceChannelStatusUpdateEvent },
        VoiceStateUpdate { t: "VOICE_STATE_UPDATE", type: VoiceStateUpdateEvent },
        WebhookUpdate { t: "WEBHOOKS_UPDATE", type: WebhooksUpdateEvent },
    },
    non_dispatch op 7, {
        GatewayReconnect { type: GatewayReconnectEvent }
    },
    non_dispatch op 11, {
        HeartbeatAck { type: HeartbeatAckEvent }
    },
    non_dispatch op 9, {
        InvalidSession { type: InvalidSessionEvent }
    }
}
