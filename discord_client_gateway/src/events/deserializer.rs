use crate::events::structs::UnknownEvent;
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use discord_client_structs::structs::user::session::Session;
use crate::events::structs::gateway::SessionsReplaceEvent;

impl<'de> Deserialize<'de> for UnknownEvent {
    fn deserialize<D>(deserializer: D) -> Result<UnknownEvent, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;
        let event_type = v
            .get("t")
            .and_then(Value::as_str)
            .unwrap_or("UNKNOWN")
            .to_string();
        let data = v.get("d").cloned().unwrap_or(Value::Null);
        let op = v.get("op").and_then(Value::as_u64).unwrap_or(0) as u8;
        Ok(UnknownEvent {
            event_type,
            data,
            op,
        })
    }
}

// The event is an array not an object...
impl<'de> Deserialize<'de> for SessionsReplaceEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let sessions = Vec::<Session>::deserialize(deserializer)?;
        Ok(SessionsReplaceEvent { sessions })
    }
}
