use crate::events::gateway::GatewayPayload;
use crate::events::structs::call::CallCreateEvent;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::*;
use crate::events::structs::channel::typing::*;
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::poll::*;
use crate::events::structs::message::reaction::*;
use crate::events::structs::message::*;
use crate::events::structs::misc::*;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use crate::events::structs::user::*;
use crate::events::structs::*;
use serde::Deserialize;

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
                        Some($t) => <$event_struct>::deserialize(payload.d).map(Event::$variant),
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
        Ready { t: "READY", type: ReadyEvent },
        ReadySupplemental { t: "READY_SUPPLEMENTAL", type: ReadySupplementalEvent },
        MessageCreate { t: "MESSAGE_CREATE", type: MessageCreateEvent },
        MessageUpdate { t: "MESSAGE_UPDATE", type: MessageUpdateEvent },
        MessageDelete { t: "MESSAGE_DELETE", type: MessageDeleteEvent },
        MessageAck { t: "MESSAGE_ACK", type: MessageAckEvent },
        MessageReactionAdd { t: "MESSAGE_REACTION_ADD", type: MessageReactionAddEvent },
        MessageReactionRemove { t: "MESSAGE_REACTION_REMOVE", type: MessageReactionRemoveEvent },
        MessageReactionAddMany { t: "MESSAGE_REACTION_ADD_MANY", type: MessageReactionAddManyEvent },
        MessageReactionRemoveEmoji { t: "MESSAGE_REACTION_REMOVE_EMOJI", type: MessageReactionRemoveEmojiEvent },
        MessageReactionRemoveAll { t: "MESSAGE_REACTION_REMOVE_ALL", type: MessageReactionRemoveAllEvent },
        PresenceUpdate { t: "PRESENCE_UPDATE", type: PresenceUpdateEvent },
        ChannelCreate { t: "CHANNEL_CREATE", type: ChannelCreateEvent },
        ChannelUpdate { t: "CHANNEL_UPDATE", type: ChannelUpdateEvent },
        ChannelDelete { t: "CHANNEL_DELETE", type: ChannelDeleteEvent },
        ThreadCreate { t: "THREAD_CREATE", type: ThreadCreateEvent },
        ThreadUpdate { t: "THREAD_UPDATE", type: ThreadUpdateEvent },
        ThreadDelete { t: "THREAD_DELETE", type: ThreadDeleteEvent },
        ThreadListSync { t: "THREAD_LIST_SYNC", type: ThreadListSyncEvent },
        ThreadMemberUpdate { t: "THREAD_MEMBER_UPDATE", type: ThreadMemberUpdateEvent },
        ThreadMembersUpdate { t: "THREAD_MEMBERS_UPDATE", type: ThreadMembersUpdateEvent },
        CallCreate { t: "CALL_CREATE", type: CallCreateEvent },
        PassiveUpdateV2 { t: "PASSIVE_UPDATE_V2", type: PassiveUpdateV2Event },
        ConversationSummaryUpdate { t: "CONVERSATION_SUMMARY_UPDATE", type: ConversationSummaryUpdateEvent },
        VoiceStateUpdate { t: "VOICE_STATE_UPDATE", type: VoiceStateUpdateEvent },
        VoiceChannelStatusUpdate { t: "VOICE_CHANNEL_STATUS_UPDATE", type: VoiceChannelStatusUpdateEvent },
        TypingStart { t: "TYPING_START", type: TypingStartEvent },
        SessionReplace { t: "SESSIONS_REPLACE", type: SessionsReplaceEvent },
        MessagePollVoteAdd { t: "MESSAGE_POLL_VOTE_ADD", type: MessagePollVoteAddEvent },
        MessagePollVoteRemove { t: "MESSAGE_POLL_VOTE_REMOVE", type: MessagePollVoteRemoveEvent },
        GuildMemberUpdate { t: "GUILD_MEMBER_UPDATE", type: GuildMemberUpdateEvent },
        GuildMemberAdd { t: "GUILD_MEMBER_ADD", type: GuildMemberAddEvent },
        GuildMemberRemove { t: "GUILD_MEMBER_REMOVE", type: GuildMemberRemoveEvent },
        GuildCreate { t: "GUILD_CREATE", type: GuildCreateEvent },
        GuildUpdate { t: "GUILD_UPDATE", type: GuildUpdateEvent },
        GuildDelete { t: "GUILD_DELETE", type: GuildDeleteEvent },
        GuildAuditLogEntryCreate { t: "GUILD_AUDIT_LOG_ENTRY_CREATE", type: GuildAuditLogEntryCreateEvent },
        ContentInventoryInboxStale { t: "CONTENT_INVENTORY_INBOX_STALE", type: ContentInventoryInboxStaleEvent },
        UserSettingsProtoUpdate { t: "USER_SETTINGS_PROTO_UPDATE", type: UserSettingsProtoUpdateEvent },
        UserConnectionsUpdate { t: "USER_CONNECTIONS_UPDATE", type: UserConnectionsUpdateEvent },
    },
    non_dispatch op 7, {
        GatewayReconnect { type: GatewayReconnectEvent }
    },
    non_dispatch op 11, {
        HeartbeatAck { type: HeartbeatAckEvent }
    }
}
