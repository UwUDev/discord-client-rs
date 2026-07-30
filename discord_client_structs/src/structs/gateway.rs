use discord_client_macros::discord_struct;

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct GatewayApplication {
    #[snowflake]
    pub id: u64,
    #[flag_enum(
        "EmbeddedReleased=1,ManagedEmoji=2,EmbeddedIap=3,GroupDmCreate=4,AutoModerationRuleCreateBadge=6,GameProfileDisabled=7,PublicOauth2Client=8,ContextlessActivity=9,GatewayPresence=12,GatewayPresenceLimited=13,GatewayGuildMembers=14,GatewayGuildMembersLimited=15,VerificationPendingGuildLimit=16,Embedded=17,GatewayMessageContent=18,GatewayMessageContentLimited=19,EmbeddedFirstParty=20,ApplicationCommandMigrated=21,ApplicationCommandBadge=23,Active=24,ActiveGracePeriod1=25,IframeModal=26,SocialLayerIntegration=27,Promoted=29,Partner=30"
    )]
    flags: u64,
}
