use discord_client_structs::structs::user::session::Session;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayPayload {
    pub op: u8,
    pub d: Value,
    pub s: Option<u64>,
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
