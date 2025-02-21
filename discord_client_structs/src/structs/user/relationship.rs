use crate::structs::user::User;
use crate::structs::user::deserialize_string_to_u64;
use discord_client_macros::CreatedAt;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct Relationship {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    r#type: u8,
    pub user: Option<User>,
    pub nickname: Option<String>,
    pub is_spam_request: Option<bool>,
    pub stranger_request: Option<bool>,
    pub user_ignored: bool,
    pub origin_application_id: Option<String>,
    pub since: Option<String>,
}

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct GameRelationship {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub application_id: String,
    r#type: u8,
    pub user: Option<User>,
    pub since: Option<String>,
    pub dm_access_type: u8,
    pub user_id: String,
}

impl Relationship {
    pub fn get_relationship_type(&self) -> RelationshipType {
        match self.r#type {
            0 => RelationshipType::None,
            1 => RelationshipType::Friend,
            2 => RelationshipType::Blocked,
            3 => RelationshipType::IncomingRequest,
            4 => RelationshipType::OutgoingRequest,
            5 => RelationshipType::Implicit,
            6 => RelationshipType::Suggestion,
            _ => RelationshipType::Unknown(self.r#type),
        }
    }
}

impl GameRelationship {
    pub fn get_relationship_type(&self) -> RelationshipType {
        match self.r#type {
            1 => RelationshipType::Friend,
            3 => RelationshipType::IncomingRequest,
            4 => RelationshipType::OutgoingRequest,
            _ => RelationshipType::Unknown(self.r#type),
        }
    }
}

pub enum RelationshipType {
    None,
    Friend,
    Blocked,
    IncomingRequest,
    OutgoingRequest,
    Implicit,
    Suggestion,
    Unknown(u8),
}
