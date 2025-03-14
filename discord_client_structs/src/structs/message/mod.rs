use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::application::IntegrationApplication;
use crate::structs::channel::PartialChannel;
use crate::structs::message::attachment::Attachment;
use crate::structs::message::call::MessageCall;
use crate::structs::message::embed::Embed;
use crate::structs::message::interaction::{MessageInteraction, MessageInteractionMetadata};
use crate::structs::message::poll::Poll;
use crate::structs::message::purchase::{
    MessageGiftInfo, MessagePurchaseNotification, MessageRoleSubscription,
};
use crate::structs::message::reaction::Reaction;
use crate::structs::message::select::SelectOption;
use crate::structs::message::soundboard::SoundboardSound;
use crate::structs::message::sticker::{Sticker, StickerItem};
use crate::structs::misc::{Emoji, Potion};
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use discord_client_macros::CreatedAt;
use serde::{Deserialize, Serialize};

pub mod attachment;
pub mod call;
pub mod embed;
pub mod interaction;
pub mod poll;
pub mod purchase;
pub mod query;
pub mod reaction;
pub mod select;
pub mod soundboard;
pub mod sticker;

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct Message {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    #[builder(default)]
    #[serde(skip_serializing)]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    #[builder(default)]
    #[serde(skip_serializing)]
    pub channel_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub guild_id: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub author: User,
    pub content: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub edited_timestamp: Option<DateTime<Utc>>,
    #[builder(default)]
    pub tts: bool,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub mention_everyone: bool,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub mentions: Option<Vec<User>>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_vec_u64_as_string")]
    pub mention_roles: Vec<u64>,
    pub mention_channels: Option<Vec<PartialChannel>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub pinned: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub webhook_id: Option<u64>,
    #[builder(default)]
    #[serde(deserialize_with = "deserialize_message_type")]
    #[serde(serialize_with = "serialize_message_type")]
    pub r#type: MessageType,
    pub activity: Option<MessageActivity>,
    pub application: Option<IntegrationApplication>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub application_id: Option<u64>,
    #[builder(default)]
    #[serde(skip_serializing)]
    pub flags: u64,
    pub message_reference: Option<MessageReference>,
    pub referenced_message: Option<Box<Message>>,
    pub message_snapshots: Option<Vec<MessageSnapshot>>,
    pub call: Option<MessageCall>,
    pub interaction: Option<MessageInteraction>,
    pub interaction_metadata: Option<MessageInteractionMetadata>,
    pub thread: Option<PartialChannel>,
    pub role_subscription_data: Option<MessageRoleSubscription>,
    pub purchase_notification: Option<MessagePurchaseNotification>,
    pub gift_info: Option<MessageGiftInfo>,
    pub components: Vec<MessageComponent>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub stickers: Option<Vec<Sticker>>,
    pub poll: Option<Poll>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub changelog_id: Option<u64>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
    pub potions: Option<Vec<Potion>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Default,
    RecipientAdd,
    RecipientRemove,
    Call,
    ChannelNameChange,
    ChannelIconChange,
    ChannelPinnedMessage,
    UserJoin,
    GuildBoost,
    GuildBoostTier1,
    GuildBoostTier2,
    GuildBoostTier3,
    ChannelFollowAdd,
    GuildDiscoveryDisqualified,
    GuildDiscoveryRequalified,
    GuildDiscoveryGracePeriodInitialWarning,
    GuildDiscoveryGracePeriodFinalWarning,
    ThreadCreated,
    Reply,
    ChatInputCommand,
    ThreadStarterMessage,
    GuildInviteReminder,
    ContextMenuCommand,
    AutoModerationAction,
    RoleSubscriptionPurchase,
    InteractionPremiumUpsell,
    StageStart,
    StageEnd,
    StageSpeaker,
    StageTopic,
    GuildApplicationPremiumSubscription,
    GuildIncidentAlertModeEnabled,
    GuildIncidentAlertModeDisabled,
    GuildIncidentReportRaid,
    GuildIncidentReportFalseAlarm,
    PurchaseNotification,
    PollResult,
    Unknown(u64),
}

impl Default for MessageType {
    fn default() -> Self {
        Self::Default
    }
}

impl MessageType {
    pub fn as_u64(&self) -> u64 {
        match *self {
            Self::Default => 0,
            Self::RecipientAdd => 1,
            Self::RecipientRemove => 2,
            Self::Call => 3,
            Self::ChannelNameChange => 4,
            Self::ChannelIconChange => 5,
            Self::ChannelPinnedMessage => 6,
            Self::UserJoin => 7,
            Self::GuildBoost => 8,
            Self::GuildBoostTier1 => 9,
            Self::GuildBoostTier2 => 10,
            Self::GuildBoostTier3 => 11,
            Self::ChannelFollowAdd => 12,
            Self::GuildDiscoveryDisqualified => 14,
            Self::GuildDiscoveryRequalified => 15,
            Self::GuildDiscoveryGracePeriodInitialWarning => 16,
            Self::GuildDiscoveryGracePeriodFinalWarning => 17,
            Self::ThreadCreated => 18,
            Self::Reply => 19,
            Self::ChatInputCommand => 20,
            Self::ThreadStarterMessage => 21,
            Self::GuildInviteReminder => 22,
            Self::ContextMenuCommand => 23,
            Self::AutoModerationAction => 24,
            Self::RoleSubscriptionPurchase => 25,
            Self::InteractionPremiumUpsell => 26,
            Self::StageStart => 27,
            Self::StageEnd => 28,
            Self::StageSpeaker => 29,
            Self::StageTopic => 31,
            Self::GuildApplicationPremiumSubscription => 32,
            Self::GuildIncidentAlertModeEnabled => 36,
            Self::GuildIncidentAlertModeDisabled => 37,
            Self::GuildIncidentReportRaid => 38,
            Self::GuildIncidentReportFalseAlarm => 39,
            Self::PurchaseNotification => 44,
            Self::PollResult => 46,
            Self::Unknown(n) => n,
        }
    }
}

impl From<u64> for MessageType {
    fn from(value: u64) -> Self {
        match value {
            0 => Self::Default,
            1 => Self::RecipientAdd,
            2 => Self::RecipientRemove,
            3 => Self::Call,
            4 => Self::ChannelNameChange,
            5 => Self::ChannelIconChange,
            6 => Self::ChannelPinnedMessage,
            7 => Self::UserJoin,
            8 => Self::GuildBoost,
            9 => Self::GuildBoostTier1,
            10 => Self::GuildBoostTier2,
            11 => Self::GuildBoostTier3,
            12 => Self::ChannelFollowAdd,
            14 => Self::GuildDiscoveryDisqualified,
            15 => Self::GuildDiscoveryRequalified,
            16 => Self::GuildDiscoveryGracePeriodInitialWarning,
            17 => Self::GuildDiscoveryGracePeriodFinalWarning,
            18 => Self::ThreadCreated,
            19 => Self::Reply,
            20 => Self::ChatInputCommand,
            21 => Self::ThreadStarterMessage,
            22 => Self::GuildInviteReminder,
            23 => Self::ContextMenuCommand,
            24 => Self::AutoModerationAction,
            25 => Self::RoleSubscriptionPurchase,
            26 => Self::InteractionPremiumUpsell,
            27 => Self::StageStart,
            28 => Self::StageEnd,
            29 => Self::StageSpeaker,
            31 => Self::StageTopic,
            32 => Self::GuildApplicationPremiumSubscription,
            36 => Self::GuildIncidentAlertModeEnabled,
            37 => Self::GuildIncidentAlertModeDisabled,
            38 => Self::GuildIncidentReportRaid,
            39 => Self::GuildIncidentReportFalseAlarm,
            44 => Self::PurchaseNotification,
            46 => Self::PollResult,
            n => Self::Unknown(n),
        }
    }
}

fn deserialize_message_type<'de, D>(deserializer: D) -> Result<MessageType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u64::deserialize(deserializer)?;
    Ok(MessageType::from(value))
}

fn serialize_message_type<S>(value: &MessageType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u64(value.as_u64())
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageActivity {
    #[serde(deserialize_with = "deserialize_message_activity_type")]
    #[serde(serialize_with = "serialize_message_activity_type")]
    pub r#type: MessageActivityType,
    pub session_id: Option<String>,
    pub party_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageActivityType {
    Join,
    Spectate,
    Listen,
    JoinRequest,
    Unknown(u8),
}

impl MessageActivityType {
    pub fn as_u8(&self) -> u8 {
        match *self {
            Self::Join => 1,
            Self::Spectate => 2,
            Self::Listen => 3,
            Self::JoinRequest => 5,
            Self::Unknown(n) => n,
        }
    }
}

impl From<u8> for MessageActivityType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Join,
            2 => Self::Spectate,
            3 => Self::Listen,
            5 => Self::JoinRequest,
            n => Self::Unknown(n),
        }
    }
}

impl Default for MessageActivityType {
    fn default() -> Self {
        Self::Join
    }
}

fn deserialize_message_activity_type<'de, D>(
    deserializer: D,
) -> Result<MessageActivityType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u8::deserialize(deserializer)?;
    Ok(MessageActivityType::from(value))
}

fn serialize_message_activity_type<S>(
    value: &MessageActivityType,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(value.as_u8())
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageReference {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub message_id: Option<u64>, // non-existent when a thread is created (wtf ?)
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub channel_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    #[builder(setter(strip_option = "false"))]
    pub guild_id: Option<u64>,
    pub forward_only: Option<MessageForwardOnly>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageForwardOnly {
    pub embed_indices: Option<Vec<u64>>,
    pub attachment_ids: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageSnapshot {
    pub message: SnapshotMessage,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct SnapshotMessage {
    pub content: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub edited_timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    pub mentions: Option<Vec<User>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    #[serde(serialize_with = "serialize_option_vec_u64_as_string")]
    pub mention_roles: Option<Vec<u64>>,
    #[serde(default)]
    pub attachments: Option<Vec<Attachment>>,
    pub embeds: Vec<Embed>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_message_type")]
    #[serde(serialize_with = "serialize_message_type")]
    pub r#type: MessageType,
    pub flags: u64,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageComponent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_component_type")]
    #[serde(serialize_with = "serialize_component_type")]
    pub r#type: ComponentType,
    pub custom_id: Option<String>,
    pub disabled: Option<bool>,
    pub style: Option<u8>,
    pub label: Option<String>,
    pub emoji: Option<Emoji>,
    pub url: Option<String>,
    pub options: Option<Vec<SelectOption>>,
    pub placeholder: Option<String>,
    pub min_values: Option<u64>,
    pub max_values: Option<u64>,
    pub components: Option<Vec<MessageComponent>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    ActionRow,
    Button,
    StringSelect,
    TextInput,
    UserSelect,
    RoleSelect,
    MentionableSelect,
    ChannelSelect,
    Unknown(u8),
}

impl ComponentType {
    pub fn as_u8(&self) -> u8 {
        match *self {
            Self::ActionRow => 1,
            Self::Button => 2,
            Self::StringSelect => 3,
            Self::TextInput => 4,
            Self::UserSelect => 5,
            Self::RoleSelect => 6,
            Self::MentionableSelect => 7,
            Self::ChannelSelect => 8,
            Self::Unknown(n) => n,
        }
    }
}

impl From<u8> for ComponentType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::ActionRow,
            2 => Self::Button,
            3 => Self::StringSelect,
            4 => Self::TextInput,
            5 => Self::UserSelect,
            6 => Self::RoleSelect,
            7 => Self::MentionableSelect,
            8 => Self::ChannelSelect,
            n => Self::Unknown(n),
        }
    }
}

impl Default for ComponentType {
    fn default() -> Self {
        Self::ActionRow
    }
}

fn deserialize_component_type<'de, D>(deserializer: D) -> Result<ComponentType, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = u8::deserialize(deserializer)?;
    Ok(ComponentType::from(value))
}

fn serialize_component_type<S>(value: &ComponentType, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_u8(value.as_u8())
}
