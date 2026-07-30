use crate::structs::message::soundboard::SoundboardSound;
use crate::structs::misc::Emoji;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct MessagePurchaseNotification {
    pub r#type: u8,
    pub guild_product_purchase: Option<GuildProductPurchase>,
}

#[discord_struct]
pub struct GuildProductPurchase {
    #[snowflake]
    pub listing_id: u64,
    pub product_name: String,
}

#[discord_struct]
pub struct MessageGiftInfo {
    pub emoji: Option<Emoji>,
    pub sound: Option<SoundboardSound>,
}

#[discord_struct]
pub struct MessageRoleSubscription {
    #[snowflake]
    pub role_subscription_listing_id: u64,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}
