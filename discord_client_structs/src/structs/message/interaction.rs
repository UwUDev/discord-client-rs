use crate::structs::user::User;
use discord_client_macros::discord_struct;
use std::collections::HashMap;

#[discord_struct]
pub struct MessageInteraction {
    #[snowflake]
    pub id: u64,
    pub r#type: u8,
    pub name: String,
    pub user: User,
}

#[discord_struct]
pub struct MessageInteractionMetadata {
    #[snowflake]
    pub id: u64,
    pub r#type: u8,
    pub name: Option<String>,
    pub command_type: Option<u8>,
    pub ephemerality_reason: Option<u8>,
    pub user: User,
    pub authorizing_integration_owners: Option<HashMap<String, String>>,
    #[serde(default)]
    #[snowflake]
    pub original_response_message_id: Option<u64>,
    pub interacted_message_id: Option<u64>,
    pub triggering_interaction_metadata: Option<Box<MessageInteractionMetadata>>,
    pub target_user: Option<User>,
    #[serde(default)]
    #[snowflake]
    pub target_message_id: Option<u64>,
}
