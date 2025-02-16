use crate::deserializer::{
    deserialize_option_iso8601_string_to_date, deserialize_option_string_to_u64,
    deserialize_string_to_u64, deserialize_string_to_vec_u64,
};
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
use serde::Deserialize;

pub mod attachment;
pub mod call;
pub mod embed;
pub mod interaction;
pub mod poll;
pub mod purchase;
pub mod reaction;
pub mod select;
pub mod soundboard;
pub mod sticker;

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub author: User,
    pub content: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub tts: bool,
    pub mention_everyone: bool,
    pub mentions: Option<Vec<User>>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub mention_roles: Vec<u64>,
    pub mention_channels: Option<Vec<PartialChannel>>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub reactions: Option<Vec<Reaction>>,
    pub nonce: Option<String>,
    pub pinned: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub webhook_id: Option<u64>,
    pub r#type: u8,
    pub activity: Option<MessageActivity>,
    pub application: Option<IntegrationApplication>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
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
    pub changelog_id: Option<u64>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
    pub potions: Option<Vec<Potion>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageActivity {
    pub r#type: u8,
    pub session_id: Option<String>,
    pub party_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageReference {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_id: Option<u64>, // non-existent when a thread is created (wtf ?)
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub forward_only: Option<MessageForwardOnly>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageForwardOnly {
    pub embed_indices: Option<Vec<u64>>,
    pub attachment_ids: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageSnapshot {
    pub content: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub timestamp: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub mentions: Vec<User>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub mention_roles: Vec<u64>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    pub r#type: u8,
    pub flags: u64,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageComponent {
    pub r#type: u8,
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
