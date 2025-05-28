use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::clan::ClanBadge;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::{CreatedAt, Flags};
use serde::{Deserialize, Serialize};

pub mod activity;
pub mod connection;
pub mod experiment;
pub mod presence;
pub mod relationship;
pub mod session;
pub mod status;

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub username: String,
    pub discriminator: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub bot: Option<bool>,
    pub system: Option<bool>,
    pub mfa_enabled: Option<bool>,
    pub banner: Option<String>,
    pub accent_color: Option<u64>,
    pub locale: Option<String>,
    pub verified: Option<bool>,
    pub email: Option<String>,
    #[flag_enum(
        "Staff=0,Partner=1,Hypesquad=2,BugHunterLevelOne=3, MfaSms=4,PremiumPromoDismissed=5,HypesquadOnlineHouse1=6,HypesquadOnlineHouse2=7, HypesquadOnlineHouse3=8,PremiumEarlySupporter=9,TeamPseudoUser=10, IsHubspotContact=11,System=12,HasUnreadUrgentMessages=13,BugHunterLevelTwo=14, UnderageDeleted=15,VerifiedBot=16,VerifiedDeveloper=17,CertifiedModerator=18, BotHttpInteractions=19,Spammer=20,DisablePremium=21,ActiveDeveloper=22, ProvisionalAccount=23,HighGlobalRateLimit=33,Deleted=34,DisabledSuspiciousActivity=35, SelfDeleted=36,PremiumDiscriminator=37,UsedDesktopClient=38,UsedWebClient=39, UsedMobileClient=40,Disabled=41,HasSessionStarted=43,Quarantined=44, PremiumEligibleForUniqueUsername=47,Collaborator=50,RestrictedCollaborator=51"
    )]
    pub flags: Option<u64>,
    pub premium_type: Option<u64>,
    #[flag_enum(
        "Staff=0,Partner=1,Hypesquad=2,BugHunterLevelOne=3, MfaSms=4,PremiumPromoDismissed=5,HypesquadOnlineHouse1=6,HypesquadOnlineHouse2=7, HypesquadOnlineHouse3=8,PremiumEarlySupporter=9,TeamPseudoUser=10, IsHubspotContact=11,System=12,HasUnreadUrgentMessages=13,BugHunterLevelTwo=14, UnderageDeleted=15,VerifiedBot=16,VerifiedDeveloper=17,CertifiedModerator=18, BotHttpInteractions=19,Spammer=20,DisablePremium=21,ActiveDeveloper=22, ProvisionalAccount=23,HighGlobalRateLimit=33,Deleted=34,DisabledSuspiciousActivity=35, SelfDeleted=36,PremiumDiscriminator=37,UsedDesktopClient=38,UsedWebClient=39, UsedMobileClient=40,Disabled=41,HasSessionStarted=43,Quarantined=44, PremiumEligibleForUniqueUsername=47,Collaborator=50,RestrictedCollaborator=51"
    )]
    pub public_flags: Option<u64>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
    pub token: Option<String>,
    pub primary_guild: Option<ClanBadge>,
    pub clan: Option<ClanBadge>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, Flags)]
#[builder(setter(into, strip_option), default)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_vec_u64_as_string")]
    pub roles: Vec<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub joined_at: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: bool,
    pub mute: bool,
    #[flag_enum(
        "DidRejoin=0,CompletedOnboarding=1,BypassesVerification=2,StartedOnboarding=3,IsGuest=4,StartedServerGuide=5,CompletedServerGuide=6,AutomodQuarantinedName=7,DmSettingsUpsellAcknowledged=9,AutomodQuarantinedClanTag=10"
    )]
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub communication_disabled_until: Option<DateTime<Utc>>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct AvatarDecorationData {
    pub asset: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub sku_id: u64,
}
