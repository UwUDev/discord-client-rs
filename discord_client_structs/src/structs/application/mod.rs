use crate::deserializer::*;
use crate::structs::application::team::{Company, Team};
use crate::structs::user::User;
use crate::structs::user::activity::EmbeddedActivityConfig;
use discord_client_macros::{EnumFromPrimitive, EnumFromString, discord_struct};
use serde::Deserialize;

pub mod team;

#[discord_struct]
pub struct IntegrationApplication {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub splash: Option<String>,
    pub r#type: Option<ApplicationType>,
    pub flags: u64,
    #[snowflake]
    pub primary_sku_id: Option<u64>,
    pub verify_key: String,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    #[snowflake]
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
    pub event_webhooks_types: Option<Vec<EventWebhookType>>,
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
    #[flag_enum(
        "Verified=0,Tag=1,Description=2,TermsOfService=3,PrivacyPolicy=4,InstallParams=5,SafeName=6,SafeDescription=7,ApprovedCommands=8,SupportGuild=9,SafeCommands=10,Mfa=11,SafeDirectoryOverview=12,SupportedLocales=13,SafeShortDescription=14,SafeRoleConnections=15"
    )]
    pub discovery_eligibility_flags: Option<u64>,
    pub is_monetized: bool,
    pub storefront_available: bool,
    pub monetization_state: Option<u64>,
    #[flag_enum(
        "Verified=0,HasTeam=1,ApprovedCommands=2,TermsOfService=3,PrivacyPolicy=4,SafeName=5,SafeDescription=6,SafeRoleConnections=7,UserIsTeamOwner=8,NotQuarantined=9,UserLocaleSupported=10,UserAgeSupported=11,UserDateOfBirthDefined=12,UserMfaEnabled=13,UserEmailVerified=14,TeamMembersEmailVerified=15,TeamMembersMfaEnabled=16,NoBlockingIssues=17,ValidPayoutStatus=18"
    )]
    pub monetization_eligibility_flags: Option<u64>,
    pub max_participants: Option<u64>,
    pub embedded_activity_config: Option<EmbeddedActivityConfig>,
}

#[discord_struct]
pub struct ApplicationExecutable {
    pub os: String,
    pub name: String,
    pub is_launcher: bool,
}

#[discord_struct]
pub struct ApplicationSku {
    pub id: Option<String>,
    pub sku: Option<String>,
    pub distributor: String,
}

#[discord_struct]
pub struct ApplicationInstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

#[discord_struct]
pub struct ApplicationIntegrationTypeConfig {
    pub oauth2_install_params: Option<ApplicationInstallParams>,
}

#[discord_struct]
pub struct ApplicationCommandIndex {
    pub applications: Vec<Application>,
    pub application_commands: Vec<ApplicationCommand>,
    #[snowflake(no_created_at)]
    pub version: u64,
}

#[discord_struct]
pub struct Application {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    #[snowflake]
    pub bot_id: Option<u64>,
    #[snowflake(no_created_at)]
    #[flag_enum(
        "EmbeddedReleased=1,ManagedEmoji=2,EmbeddedIap=3,GroupDmCreate=4,AutoModerationRuleCreateBadge=6,GameProfileDisabled=7,PublicOauth2Client=8,ContextlessActivity=9,GatewayPresence=12,GatewayPresenceLimited=13,GatewayGuildMembers=14,GatewayGuildMembersLimited=15,VerificationPendingGuildLimit=16,Embedded=17,GatewayMessageContent=18,GatewayMessageContentLimited=19,EmbeddedFirstParty=20,ApplicationCommandMigrated=21,ApplicationCommandBadge=23,Active=24,ActiveGracePeriod1=25,IframeModal=26,SocialLayerIntegration=27,Promoted=29,Partner=30"
    )]
    pub flags: u64,
}

#[discord_struct]
pub struct ApplicationCommand {
    #[snowflake]
    pub id: u64,
    pub r#type: ApplicationCommandType,
    #[snowflake]
    pub application_id: u64,
    #[snowflake(no_created_at)]
    pub version: u64,
    pub name: String,
    #[serde(default)]
    pub dm_permission: Option<bool>,
    #[serde(default)]
    pub contexts: Option<Vec<u8>>,
    #[serde(default)]
    pub integration_types: Option<Vec<u8>>,
    #[serde(default)]
    pub handler: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum ApplicationCommandType {
    #[default]
    ChatInput = 1,
    User = 2,
    Message = 3,
    Unknown(u16),
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum ApplicationType {
    #[default]
    Game = 1,
    Music = 2, // Should be disabled by discord
    TicketedEvents = 3,
    CreatorMonetization = 4,
    Unknown(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromString)]
pub enum EventWebhookType {
    #[str_value("APPLICATION_AUTHORIZED")]
    ApplicationAuthorized,
    #[str_value("ENTITLEMENT_CREATE")]
    EntitlementCreate,
    #[str_value("QUEST_USER_ENROLLMENT")]
    QuestUserEnrollment,
    Unknown,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationCommandPermission {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub r#type: u8,
    pub permission: bool,
}

#[cfg(test)]
mod discord_struct_tests {
    use super::Application;

    #[test]
    fn application_snowflake_roundtrip_and_created_at() {
        let raw =
            r#"{"id":"175928847299117063","name":"x","bot_id":"175928847299117064","flags":"0"}"#;
        let a: Application = serde_json::from_str(raw).unwrap();
        assert_eq!(a.id, 175928847299117063);
        assert_eq!(a.bot_id, Some(175928847299117064));
        assert!(a.created_at().is_some());
        assert!(a.bot_created_at().is_some());
        let out = serde_json::to_string(&a).unwrap();
        assert!(out.contains(r#""id":"175928847299117063""#), "{out}");
        assert!(out.contains(r#""bot_id":"175928847299117064""#), "{out}");
    }
}

#[cfg(test)]
mod snowflake_vec_tests {
    use discord_client_macros::discord_struct;

    #[discord_struct(no_builder, no_default)]
    struct VecSnow {
        #[snowflake]
        ids: Vec<u64>,
        #[snowflake]
        maybe: Option<Vec<u64>>,
    }

    #[test]
    fn vec_snowflake_roundtrip() {
        let raw = r#"{"ids":["1","2","3"],"maybe":["4"]}"#;
        let v: VecSnow = serde_json::from_str(raw).unwrap();
        assert_eq!(v.ids, vec![1, 2, 3]);
        assert_eq!(v.maybe, Some(vec![4]));
        let out = serde_json::to_string(&v).unwrap();
        assert!(out.contains(r#""ids":["1","2","3"]"#), "{out}");
        assert!(out.contains(r#""maybe":["4"]"#), "{out}");
    }
}
