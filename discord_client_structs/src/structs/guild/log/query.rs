use derive_builder::Builder;
use std::collections::HashMap;

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct AuditLogQuery {
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub limit: u8,
    pub user_id: Option<u64>,
    pub target_id: Option<u64>,
    pub action_type: Option<AuditLogActionType>,
}

impl AuditLogQuery {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(before) = self.before {
            map.insert("before".to_string(), before.to_string());
        }
        if let Some(after) = self.after {
            map.insert("after".to_string(), after.to_string());
        }
        let limit = if self.limit == 0u8 || self.limit > 100u8 {
            50u8
        } else {
            self.limit
        };
        map.insert("limit".to_string(), limit.to_string());
        if let Some(user_id) = self.user_id {
            map.insert("user_id".to_string(), user_id.to_string());
        }
        if let Some(target_id) = self.target_id {
            map.insert("target_id".to_string(), target_id.to_string());
        }
        if let Some(action_type) = self.action_type {
            map.insert("action_type".to_string(), (action_type as u8).to_string());
        }
        map
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditLogActionType {
    GuildUpdate = 1,
    ChannelCreate = 10,
    ChannelUpdate = 11,
    ChannelDelete = 12,
    ChannelOverwriteCreate = 13,
    ChannelOverwriteUpdate = 14,
    ChannelOverwriteDelete = 15,
    MemberKick = 20,
    MemberPrune = 21,
    MemberBanAdd = 22,
    MemberBanRemove = 23,
    MemberUpdate = 24,
    MemberRoleUpdate = 25,
    MemberMove = 26,
    MemberDisconnect = 27,
    BotAdd = 28,
    RoleCreate = 30,
    RoleUpdate = 31,
    RoleDelete = 32,
    InviteCreate = 40,
    InviteUpdate = 41,
    InviteDelete = 42,
    WebhookCreate = 50,
    WebhookUpdate = 51,
    WebhookDelete = 52,
    EmojiCreate = 60,
    EmojiUpdate = 61,
    EmojiDelete = 62,
    MessageDelete = 72,
    MessageBulkDelete = 73,
    MessagePin = 74,
    MessageUnpin = 75,
    IntegrationCreate = 80,
    IntegrationUpdate = 81,
    IntegrationDelete = 82,
    StageInstanceCreate = 83,
    StageInstanceUpdate = 84,
    StageInstanceDelete = 85,
    StickerCreate = 90,
    StickerUpdate = 91,
    StickerDelete = 92,
    GuildScheduledEventCreate = 100,
    GuildScheduledEventUpdate = 101,
    GuildScheduledEventDelete = 102,
    ThreadCreate = 110,
    ThreadUpdate = 111,
    ThreadDelete = 112,
    ApplicationCommandPermissionUpdate = 121,
    SoundboardSoundCreate = 130,
    SoundboardSoundUpdate = 131,
    SoundboardSoundDelete = 132,
    AutoModerationRuleCreate = 140,
    AutoModerationRuleUpdate = 141,
    AutoModerationRuleDelete = 142,
    AutoModerationBlockMessage = 143,
    AutoModerationFlagToChannel = 144,
    AutoModerationUserCommunicationDisabled = 145,
    AutoModerationQuarantineUser = 146,
    CreatorMonetizationRequestCreated = 150,
    CreatorMonetizationTermsAccepted = 151,
    OnboardingPromptCreate = 163,
    OnboardingPromptUpdate = 164,
    OnboardingPromptDelete = 165,
    OnboardingCreate = 166,
    OnboardingUpdate = 167,
    HarmfulLinksBlockedMessage = 180,
    VoiceChannelStatusCreate = 192,
    VoiceChannelStatusDelete = 193,
    ClydeAiProfileUpdate = 194,
    GuildScheduledEventExceptionCreate = 200,
    GuildScheduledEventExceptionUpdate = 201,
    GuildScheduledEventExceptionDelete = 202,
    GuildMemberVerificationUpdate = 210,
}
