use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::message::soundboard::SoundboardSound;
use crate::structs::misc::Emoji;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessagePurchaseNotification {
    pub r#type: u8,
    pub guild_product_purchase: Option<GuildProductPurchase>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct GuildProductPurchase {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub listing_id: u64,
    pub product_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageGiftInfo {
    pub emoji: Option<Emoji>,
    pub sound: Option<SoundboardSound>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageRoleSubscription {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub role_subscription_listing_id: u64,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}
