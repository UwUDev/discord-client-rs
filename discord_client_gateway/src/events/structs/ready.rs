use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ReadyEvent {
    pub session_id: String,
    pub analytics_token: String,
    pub auth_session_id_hash: String,
}
