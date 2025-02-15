use crate::deserializer::deserialize_option_string_to_u64;
use serde::Deserialize;
use crate::structs::user::activity::Activity;
use crate::structs::user::User;

#[derive(Debug, Deserialize, Clone)]
pub struct Presence {
    #[serde(flatten)]
    pub user: User,
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