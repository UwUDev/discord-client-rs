use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::user::User;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageInteraction {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub r#type: u8,
    pub name: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageInteractionMetadata {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub r#type: u8,
    pub name: Option<String>,
    pub command_type: Option<u8>,
    pub ephemerality_reason: Option<u8>,
    pub user: User,
    pub authorizing_integration_owners: Option<HashMap<String, String>>,
    pub original_response_message_id: Option<u64>,
    pub interacted_message_id: Option<u64>,
    pub triggering_interaction_metadata: Option<Box<MessageInteractionMetadata>>,
    pub target_user: Option<User>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub target_message_id: Option<u64>,
}
