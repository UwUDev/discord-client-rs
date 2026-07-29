use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::structs::user::Authenticator;
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

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteCommandEvent {
    #[serde(flatten)]
    pub payload: Value, // Can be whatever you've sent through the command
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthSessionChangeEvent {
    pub auth_session_id_hash: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticatorCreateEvent {
    #[serde(flatten)]
    pub authenticator: Authenticator,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticatorUpdateEvent {
    #[serde(flatten)]
    pub authenticator: Authenticator,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthenticatorDeleteEvent {
    pub id: String,
    pub r#type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2TokenRevokeEvent {
    pub access_token: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

impl<'de> Deserialize<'de> for InvalidSessionEvent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = bool::deserialize(deserializer)?;
        Ok(InvalidSessionEvent { resumable: value })
    }
}
