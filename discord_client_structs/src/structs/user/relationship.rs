use crate::structs::user::User;
use crate::structs::user::deserialize_string_to_u64;
use discord_client_macros::{CreatedAt, EnumFromPrimitive};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct Relationship {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub r#type: RelationshipType,
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
    pub r#type: RelationshipType,
    pub user: Option<User>,
    pub since: Option<String>,
    pub dm_access_type: u8,
    pub user_id: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FriendSuggestionReason {
    pub r#type: u8,
    pub platform: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FriendSuggestion {
    pub suggested_user: User,
    #[serde(default)]
    pub reasons: Vec<FriendSuggestionReason>,
    #[serde(default)]
    pub from_suggested_user_contacts: Option<bool>,
    #[serde(default)]
    pub mutual_friends_count: Option<u32>,
    #[serde(default)]
    pub contact_names: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum RelationshipType {
    None = 0,
    Friend = 1,
    Blocked = 2,
    IncomingRequest = 3,
    OutgoingRequest = 4,
    Implicit = 5,
    Suggestion = 6,
    Unknown(u16),
}
