use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::user::relationship::{
    FriendSuggestion, GameRelationship, Relationship,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RelationshipAddEvent {
    #[serde(flatten)]
    pub relationship: Relationship,
    #[serde(default)]
    pub should_notify: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RelationshipUpdateEvent {
    #[serde(flatten)]
    pub relationship: Relationship,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RelationshipRemoveEvent {
    #[serde(flatten)]
    pub relationship: Relationship,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameRelationshipAddEvent {
    #[serde(flatten)]
    pub relationship: GameRelationship,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameRelationshipRemoveEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
    pub r#type: u8,
    pub since: Option<String>,
    pub dm_access_type: u8,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FriendSuggestionCreateEvent {
    #[serde(flatten)]
    pub suggestion: FriendSuggestion,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FriendSuggestionDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub suggested_user_id: u64,
}
