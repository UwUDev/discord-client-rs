use crate::deserializer::deserialize_option_string_to_u64;
use crate::structs::user::User;
use crate::structs::user::activity::Activity;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Presence {
    #[serde(flatten)]
    pub user: Option<User>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
    pub embedded: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MergedPresences {
    pub friends: Vec<Presence>,
    pub guilds: Vec<Vec<Presence>>,
}
