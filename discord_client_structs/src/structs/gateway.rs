use crate::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayApplicationFlags {
    EmbeddedReleased,
    ManagedEmoji,
    EmbeddedIap,
    GroupDmCreate,
    RpcPrivateBeta,
    AutoModerationRuleCreateBadge,
    GameProfileDisabled,
    PublicOauth2Client,
    ContextlessActivity,
    AllowAssets,
    AllowActivityActionSpectate,
    AllowActivityActionJoinRequest,
    RpcHasConnected,
    GatewayPresence,
    GatewayPresenceLimited,
    GatewayGuildMembers,
    GatewayGuildMembersLimited,
    VerificationPendingGuildLimit,
    Embedded,
    GatewayMessageContent,
    GatewayMessageContentLimited,
    EmbeddedFirstParty,
    ApplicationCommandMigrated,
    ApplicationCommandBadge,
    Active,
    ActiveGracePeriod1,
    IframeModal,
    SocialLayerIntegration,
    Promoted,
    Partner,
    Unknown(u8),
}

impl GatewayApplicationFlags {
    fn bit_shift(&self) -> u32 {
        match self {
            Self::EmbeddedReleased => 1,
            Self::ManagedEmoji => 2,
            Self::EmbeddedIap => 3,
            Self::GroupDmCreate => 4,
            Self::RpcPrivateBeta => 5,
            Self::AutoModerationRuleCreateBadge => 6,
            Self::GameProfileDisabled => 7,
            Self::PublicOauth2Client => 8,
            Self::ContextlessActivity => 9,
            Self::AllowAssets => 8,
            Self::AllowActivityActionSpectate => 9,
            Self::AllowActivityActionJoinRequest => 10,
            Self::RpcHasConnected => 11,
            Self::GatewayPresence => 12,
            Self::GatewayPresenceLimited => 13,
            Self::GatewayGuildMembers => 14,
            Self::GatewayGuildMembersLimited => 15,
            Self::VerificationPendingGuildLimit => 16,
            Self::Embedded => 17,
            Self::GatewayMessageContent => 18,
            Self::GatewayMessageContentLimited => 19,
            Self::EmbeddedFirstParty => 20,
            Self::ApplicationCommandMigrated => 21,
            Self::ApplicationCommandBadge => 23,
            Self::Active => 24,
            Self::ActiveGracePeriod1 => 25,
            Self::IframeModal => 26,
            Self::SocialLayerIntegration => 29,
            Self::Promoted => 30,
            Self::Partner => 31,
            Self::Unknown(n) => *n as u32,
        }
    }
}

impl From<u64> for GatewayApplicationFlags {
    fn from(shift: u64) -> Self {
        match shift {
            1 => Self::EmbeddedReleased,
            2 => Self::ManagedEmoji,
            3 => Self::EmbeddedIap,
            4 => Self::GroupDmCreate,
            5 => Self::RpcPrivateBeta,
            6 => Self::AutoModerationRuleCreateBadge,
            7 => Self::GameProfileDisabled,
            8 => Self::PublicOauth2Client,
            9 => Self::ContextlessActivity,
            10 => Self::AllowAssets,
            11 => Self::AllowActivityActionSpectate,
            12 => Self::AllowActivityActionJoinRequest,
            13 => Self::RpcHasConnected,
            14 => Self::GatewayPresence,
            15 => Self::GatewayPresenceLimited,
            16 => Self::GatewayGuildMembers,
            17 => Self::GatewayGuildMembersLimited,
            18 => Self::VerificationPendingGuildLimit,
            19 => Self::Embedded,
            20 => Self::GatewayMessageContent,
            21 => Self::GatewayMessageContentLimited,
            22 => Self::EmbeddedFirstParty,
            23 => Self::ApplicationCommandMigrated,
            24 => Self::ApplicationCommandBadge,
            25 => Self::Active,
            26 => Self::ActiveGracePeriod1,
            27 => Self::IframeModal,
            29 => Self::SocialLayerIntegration,
            30 => Self::Promoted,
            31 => Self::Partner,
            n => Self::Unknown(n as u8),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GatewayApplication {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    flags: u64,
}

impl GatewayApplication {
    pub fn get_flags(&self) -> Vec<GatewayApplicationFlags> {
        (0..64)
            .filter_map(|shift| {
                let mask = 1u64.wrapping_shl(shift);
                if (self.flags & mask) != 0 {
                    Some(GatewayApplicationFlags::from(shift as u64))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn has_flag(&self, flag: GatewayApplicationFlags) -> bool {
        (self.flags & (1u64 << flag.bit_shift())) != 0
    }
}
