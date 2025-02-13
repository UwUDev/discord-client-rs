use crate::events::structs::*;
use serde::Deserialize;

pub(crate) mod deserializer;
pub mod structs;

macro_rules! define_events {
    ($(
        $variant:ident => $event_str:expr, $event_struct:ty
    ),+ $(,)?) => {
        #[derive(Debug, Clone)]
        pub enum Event {
            $(
                $variant($event_struct),
            )+
            Unknown(UnknownEvent),
        }

        impl Event {
            pub fn event_name(&self) -> &str {
                match self {
                    $(
                        Event::$variant(_) => $event_str,
                    )+
                    Event::Unknown(unknown) => &unknown.event_type,
                }
            }
        }

        impl std::fmt::Display for Event {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Event::$variant(_) => write!(f, "{}", $event_str),
                    )+
                    Event::Unknown(unknown) => write!(f, "Unknown (op {}): {}",unknown.op, unknown.event_type),
                }
            }
        }
    }
}

define_events! {
    Ready => "READY", ReadyEvent,
    MessageCreate => "MESSAGE_CREATE", MessageCreateEvent,
    MessageReactionAdd => "MESSAGE_REACTION_ADD", MessageReactionAddEvent,
}

macro_rules! parse_dispatch_event {
    ($payload:expr, $( $variant:ident => $event_str:expr, $event_struct:ty ),+ $(,)?) => {{
        match $payload.t.as_deref() {
            $(
                Some($event_str) => {
                    let event = <$event_struct>::deserialize($payload.d)
                        .map(Event::$variant)?;
                    Ok(event)
                },
            )+
            Some(other) => Ok(Event::Unknown(UnknownEvent {
                event_type: other.to_string(),
                data: $payload.d,
                op: $payload.op,
            })),
            None => Err(serde::de::Error::custom("Dispatch event missing 't' field")),
        }
    }};
}

pub fn parse_gateway_payload(payload: GatewayPayload) -> Result<Event, serde_json::Error> {
    if payload.op == 0 {
        parse_dispatch_event!(payload,
            Ready => "READY", ReadyEvent,
            MessageCreate => "MESSAGE_CREATE", MessageCreateEvent,
            MessageReactionAdd => "MESSAGE_REACTION_ADD", MessageReactionAddEvent,
        )
    } else {
        Ok(Event::Unknown(UnknownEvent {
            event_type: "UNKNOWN".to_string(),
            data: payload.d,
            op: payload.op,
        }))
    }
}
