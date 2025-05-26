pub mod direct_message;
pub mod note;
pub mod relationship;

use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::structs::user::connection::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UserConnectionsUpdateEvent {
    #[serde(flatten)]
    pub connection: Option<Connection>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>, // null if it's yourself
}
