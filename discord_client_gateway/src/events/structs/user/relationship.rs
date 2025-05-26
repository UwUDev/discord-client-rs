use discord_client_structs::structs::user::relationship::Relationship;
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
