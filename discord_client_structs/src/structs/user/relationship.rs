use crate::structs::user::User;
use discord_client_macros::{EnumFromPrimitive, discord_struct};
use serde::Deserialize;

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct Relationship {
    #[snowflake]
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

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct GameRelationship {
    #[snowflake]
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
