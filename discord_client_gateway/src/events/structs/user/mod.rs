pub mod direct_message;
pub mod note;
pub mod relationship;

use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::user::connection::Connection;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct UserConnectionsUpdateEvent {
    #[serde(flatten)]
    pub connection: Option<Connection>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>, // null if it's yourself
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserApplicationUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserApplicationRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserMergeOperationCompletedEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub merge_operation_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub source_user_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserRequiredActionUpdateEvent {
    pub required_action: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserSettingsUpdateEvent {
    pub settings: Value,
}
