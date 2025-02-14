use discord_client_structs::structs::user::activity::Activity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PresenceUpdateEvent {
    pub since: Option<u64>,
    pub activities: Vec<Activity>,
    pub status: String,
    pub afk: Option<bool>,
}
