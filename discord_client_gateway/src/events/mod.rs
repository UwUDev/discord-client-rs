use crate::events::structs::*;
use serde::Deserialize;

pub(crate) mod deserializer;
pub mod structs;

macro_rules! define_events {
    (
        dispatch op $dispatch_op:expr, {
            $( $variant:ident { t: $t:expr, type: $event_struct:ty } ),+
        },
        non_dispatch op $non_dispatch_op:expr, {
            $( $variant_nd:ident { type: $event_struct_nd:ty } ),+
        },
    ) => {
        #[derive(Debug, Clone)]
        pub enum Event {
            $(
                $variant($event_struct),
            )+
            $(
                $variant_nd($event_struct_nd),
            )+
            Unknown(UnknownEvent),
        }

        impl Event {
            pub fn event_name(&self) -> &str {
                match self {
                    $(
                        Event::$variant(_) => $t,
                    )+
                    $(
                        Event::$variant_nd(_) => stringify!($variant_nd),
                    )+
                    Event::Unknown(unknown) => &unknown.event_type,
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
                        Event::$variant_nd(_) => write!(f, "{}", stringify!($variant_nd)),
                    )+
                    Event::Unknown(unknown) => write!(f, "Unknown ({}): {}",unknown.op, unknown.event_type),
                }
            }
        }

        pub fn parse_gateway_payload(payload: GatewayPayload) -> Result<Event, serde_json::Error> {
            match payload.op {
                $dispatch_op => {
                    match payload.t.as_deref() {
                        $(
                            Some($t) => {
                                return <$event_struct>::deserialize(payload.d).map(Event::$variant);
                            },
                        )+
                        Some(other) => {
                            Ok(Event::Unknown(UnknownEvent {
                                event_type: other.to_string(),
                                data: payload.d,
                                op: payload.op,
                            }))
                        },
                        None => return Err(serde::de::Error::custom("Dispatch event missing 't' field")),
                    }
                },
                $non_dispatch_op => {
                    $(
                        return <$event_struct_nd>::deserialize(payload.d).map(Event::$variant_nd);
                    )+
                },
                _ => {
                    Ok(Event::Unknown(UnknownEvent {
                        event_type: "UNKNOWN".to_string(),
                        data: payload.d,
                        op: payload.op,
                    }))
                }
            }
        }
    };
}

define_events! {
    dispatch op 0, {
        Ready { t: "READY", type: ReadyEvent },
        MessageCreate { t: "MESSAGE_CREATE", type: MessageCreateEvent },
        MessageUpdate { t: "MESSAGE_UPDATE", type: MessageUpdateEvent },
        MessageDelete { t: "MESSAGE_DELETE", type: MessageDeleteEvent },
        MessageReactionAdd { t: "MESSAGE_REACTION_ADD", type: MessageReactionAddEvent },
        PresenceUpdate { t: "PRESENCE_UPDATE", type: PresenceUpdateEvent },
        ChannelCreate { t: "CHANNEL_CREATE", type: ChannelCreateEvent },
        ChannelUpdate { t: "CHANNEL_UPDATE", type: ChannelUpdateEvent },
        ChannelDelete { t: "CHANNEL_DELETE", type: ChannelDeleteEvent },
        ThreadCreate { t: "THREAD_CREATE", type: ThreadCreateEvent },
        ThreadUpdate { t: "THREAD_UPDATE", type: ThreadUpdateEvent },
        ThreadDelete { t: "THREAD_DELETE", type: ThreadDeleteEvent },
        ThreadListSync { t: "THREAD_LIST_SYNC", type: ThreadListSyncEvent }
    },
    non_dispatch op 7, {
        GatewayReconnect { type: GatewayReconnectEvent }
    },
}
