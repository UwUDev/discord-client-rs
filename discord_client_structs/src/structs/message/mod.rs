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
use discord_client_macros::{CreatedAt, EnumFromPrimitive};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    Default = 0,
    RecipientAdd = 1,
    RecipientRemove = 2,
    Call = 3,
    ChannelNameChange = 4,
    ChannelIconChange = 5,
    ChannelPinnedMessage = 6,
    UserJoin = 7,
    GuildBoost = 8,
    GuildBoostTier1 = 9,
    GuildBoostTier2 = 10,
    GuildBoostTier3 = 11,
    ChannelFollowAdd = 12,
    GuildDiscoveryDisqualified = 14,
    GuildDiscoveryRequalified = 15,
    GuildDiscoveryGracePeriodInitialWarning = 16,
    GuildDiscoveryGracePeriodFinalWarning = 17,
    ThreadCreated = 18,
    Reply = 19,
    ChatInputCommand = 20,
    ThreadStarterMessage = 21,
    GuildInviteReminder = 22,
    ContextMenuCommand = 23,
    AutoModerationAction = 24,
    RoleSubscriptionPurchase = 25,
    InteractionPremiumUpsell = 26,
    StageStart = 27,
    StageEnd = 28,
    StageSpeaker = 29,
    StageTopic = 31,
    GuildApplicationPremiumSubscription = 32,
    GuildIncidentAlertModeEnabled = 36,
    GuildIncidentAlertModeDisabled = 37,
    GuildIncidentReportRaid = 38,
    GuildIncidentReportFalseAlarm = 39,
    PurchaseNotification = 44,
    PollResult = 46,
    Unknown(u8),
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageActivity {
    pub r#type: MessageActivityType,
    pub session_id: Option<String>,
    pub party_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum MessageActivityType {
    #[default]
    Join = 1,
    Spectate = 2,
    Listen = 3,
    JoinRequest = 5,
    Unknown(u8),
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
    pub r#type: MessageType,
    pub flags: u64,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageComponent {
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

#[derive(Debug, Clone, PartialEq, Eq, EnumFromPrimitive)]
#[repr(u8)]
pub enum ComponentType {
    #[default]
    ActionRow = 1,
    Button = 2,
    StringSelect = 3,
    TextInput = 4,
    UserSelect = 5,
    RoleSelect = 6,
    MentionableSelect = 7,
    ChannelSelect = 8,
    Unknown(u8),
}
