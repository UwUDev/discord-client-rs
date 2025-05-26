use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::application::team::{Company, Team};
use crate::structs::user::User;
use crate::structs::user::activity::EmbeddedActivityConfig;
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

pub mod team;

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct IntegrationApplication {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub splash: Option<String>,
    pub r#type: Option<u64>,
    pub flags: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub primary_sku_id: Option<u64>,
    pub verify_key: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub eula_id: Option<u64>,
    pub slug: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub executables: Option<Vec<ApplicationExecutable>>,
    pub third_party_skus: Option<Vec<ApplicationSku>>,
    pub hook: bool,
    pub overlay: bool,
    pub overlay_methods: Option<u64>,
    pub overlay_warn: bool,
    pub overlay_compatibility_hook: bool,
    pub bot: Option<User>,
    pub owner: User,
    pub team: Option<Team>,
    pub developers: Option<Vec<Company>>,
    pub publishers: Option<Vec<Company>>,
    pub rpc_origins: Option<Vec<String>>,
    pub redirect_uris: Vec<String>,
    pub deeplink_uri: Option<String>,
    pub integration_public: bool,
    pub integration_require_code_grant: bool,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub bot_disabled: bool,
    pub bot_quarantined: bool,
    pub approximate_guild_count: Option<u64>,
    pub approximate_user_install_count: u64,
    pub internal_guild_restriction: u64,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
    pub interactions_endpoint_url: String,
    pub interactions_version: u64,
    pub interactions_event_types: Vec<String>,
    pub event_webhooks_status: Option<u64>,
    pub event_webhooks_url: Option<String>,
    pub event_webhooks_types: Option<Vec<String>>,
    pub explicit_content_filter: u64,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<ApplicationInstallParams>,
    pub custom_install_url: Option<String>,
    pub integration_types_config: Option<Vec<ApplicationIntegrationTypeConfig>>,
    pub is_verified: bool,
    pub verification_state: Option<u64>,
    pub store_application_state: Option<u64>,
    pub rpc_application_state: Option<u64>,
    pub creator_monetization_state: Option<u64>,
    pub is_discoverable: bool,
    pub discoverability_state: Option<u64>,
    pub discovery_eligibility_flags: Option<u64>,
    pub is_monetized: bool,
    pub storefront_available: bool,
    pub monetization_state: Option<u64>,
    pub monetization_eligibility_flags: Option<u64>,
    pub max_participants: Option<u64>,
    pub embedded_activity_config: Option<EmbeddedActivityConfig>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationExecutable {
    pub os: String,
    pub name: String,
    pub is_launcher: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationSku {
    pub id: Option<String>,
    pub sku: Option<String>,
    pub distributor: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationInstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationIntegrationTypeConfig {
    pub oauth2_install_params: Option<ApplicationInstallParams>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationCommandIndex {
    pub applications: Vec<Application>,
    pub application_commands: Vec<ApplicationCommand>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub version: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct Application {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub bot_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub flags: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct ApplicationCommand {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_application_command_type")]
    #[serde(serialize_with = "serialize_application_command_type")]
    pub r#type: ApplicationCommandType, // todo: parse type
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub application_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub version: u64,
    pub name: String,
    pub dm_permission: bool,
    pub contexts: Vec<u8>,
    pub integration_types: Vec<u8>,
    pub handler: u8,
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApplicationCommandType {
    ChatInput,
    User,
    Message,
    Unknown(u8),
}

impl ApplicationCommandType {
    pub fn as_u8(&self) -> u8 {
        match self {
            ApplicationCommandType::ChatInput => 1,
            ApplicationCommandType::User => 2,
            ApplicationCommandType::Message => 3,
            ApplicationCommandType::Unknown(v) => *v,
        }
    }
}

impl From<u8> for ApplicationCommandType {
    fn from(value: u8) -> Self {
        match value {
            1 => ApplicationCommandType::ChatInput,
            2 => ApplicationCommandType::User,
            3 => ApplicationCommandType::Message,
            _ => ApplicationCommandType::Unknown(value),
        }
    }
}

impl Default for ApplicationCommandType {
    fn default() -> Self {
        ApplicationCommandType::Unknown(0)
    }
}

fn deserialize_application_command_type<'de, D>(deserializer: D) -> Result<ApplicationCommandType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: u8 = Deserialize::deserialize(deserializer)?;
    Ok(ApplicationCommandType::from(value))
}

fn serialize_application_command_type<S>(command_type: &ApplicationCommandType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(command_type.as_u8())
}