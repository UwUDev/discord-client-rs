use discord_client_structs::structs::user::session::Session;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u32>,
    pub t: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayReconnectEvent;

#[derive(Debug, Deserialize, Clone)]
pub struct HeartbeatAckEvent;

#[derive(Debug, Clone)]
pub struct SessionsReplaceEvent {
    pub sessions: Vec<Session>,
}

#[derive(Debug, Clone)]
pub struct InvalidSessionEvent {
    pub resumable: bool,
}

impl<'de> Deserialize<'de> for InvalidSessionEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(InvalidSessionEvent {
            resumable: value
        })
    }
}