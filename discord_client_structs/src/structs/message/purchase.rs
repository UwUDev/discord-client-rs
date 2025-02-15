use crate::deserializer::deserialize_string_to_u64;
use crate::structs::message::soundboard::SoundboardSound;
use crate::structs::misc::Emoji;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct MessagePurchaseNotification {
    pub r#type: u8,
    pub guild_product_purchase: Option<GuildProductPurchase>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildProductPurchase {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub listing_id: u64,
    pub product_name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageGiftInfo {
    pub emoji: Option<Emoji>,
    pub sound: Option<SoundboardSound>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageRoleSubscription {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub role_subscription_listing_id: u64,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}
