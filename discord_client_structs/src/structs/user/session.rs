use crate::structs::user::activity::Activity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    pub session_id: String,
    pub client_info: ClientInfo,
    pub status: String,
    pub activities: Vec<Activity>,
    pub active: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientInfo {
    pub client: String,
    pub os: String,
    pub version: u64,
}
