use crate::deserializer::{
    deserialize_iso8601_string_to_date, deserialize_option_iso8601_string_to_date,
    deserialize_option_string_to_u64, deserialize_option_string_to_vec_u64,
    deserialize_string_to_u64, deserialize_string_to_vec_u64,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    pub name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub roles: Option<Vec<u64>>,
    pub user: Option<User>,
    pub require_colons: Option<bool>,
    pub managed: Option<bool>,
    pub animated: Option<bool>,
    pub available: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Member {
    pub user: Option<User>,
    pub nick: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub roles: Vec<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub joined_at: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub premium_since: Option<DateTime<Utc>>,
    pub deaf: bool,
    pub mute: bool,
    pub flags: u64,
    pub pending: Option<bool>,
    pub permissions: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub communication_disabled_until: Option<DateTime<Utc>>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AvatarDecorationData {
    pub asset: String,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sku_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
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
    pub flags: Option<u64>,
    pub premium_type: Option<u64>,
    pub public_flags: Option<u64>,
    pub avatar_decoration_data: Option<AvatarDecorationData>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Activity {
    pub name: String,
    #[serde(rename = "type")]
    pub activity_type: u8,
    pub url: Option<String>,
    pub created_at: u64,
    pub timestamps: Option<ActivityTimestamp>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub details: Option<String>,
    pub state: Option<String>,
    pub emoji: Option<Emoji>,
    pub party: Option<Party>,
    pub assets: Option<ActivityAsset>,
    pub secrets: Option<ActivitySecrets>,
    pub instance: Option<bool>,
    pub flags: Option<u64>,
    pub id: Option<String>,
    pub buttons: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityTimestamp {
    pub start: Option<u64>,
    pub end: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Party {
    pub id: Option<String>,
    pub size: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityAsset {
    pub large_image: Option<String>,
    pub large_text: Option<String>,
    pub small_image: Option<String>,
    pub small_text: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivitySecrets {
    pub join: Option<String>,
    pub spectate: Option<String>,
    #[serde(rename = "match")]
    pub instanced_match: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Message {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub author: User,
    pub content: String,
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
    #[serde(rename = "type")]
    pub message_type: u8,
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
pub struct PartialChannel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub channel_type: u8,
    pub name: Option<String>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Attachment {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub filename: String,
    pub title: Option<String>,
    pub uploaded_filename: Option<String>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub size: u64,
    pub url: String,
    pub proxy_url: String,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
    pub ephemeral: Option<bool>,
    pub duration_secs: Option<f64>,
    pub waveform: Option<String>,
    pub flags: Option<u64>,
    pub is_clip: Option<bool>,
    pub is_thumbnail: Option<bool>,
    pub is_remix: Option<bool>,
    pub is_spoiler: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub clip_created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub clip_participant_ids: Option<Vec<u64>>,
    pub clip_participants: Option<Vec<User>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub application: Option<IntegrationApplication>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationApplication {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub description: String,
    pub icon: Option<String>,
    pub cover_image: Option<String>,
    pub splash: Option<String>,
    pub r#type: Option<u64>,
    pub flags: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub primary_sku_id: Option<u64>,
    pub verify_key: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub eula_id: Option<u64>,
    pub slug: Option<String>,
    pub aliases: Option<Vec<String>>,
    pub executables: Option<Vec<ApplicationExecutable>>,
    pub third_party_skus: Option<Vec<ApplicationSku>>,
    pub hook: bool,
    pub overlay: bool,
    pub overlay_methods: Option<u64>,
    pub overlay_warn: bool,
    pub overlay_compatibility_hook: bool,
    pub bot: Option<User>,
    pub owner: User,
    pub team: Option<Team>,
    pub developers: Option<Vec<Company>>,
    pub publishers: Option<Vec<Company>>,
    pub rpc_origins: Option<Vec<String>>,
    pub redirect_uris: Vec<String>,
    pub deeplink_uri: Option<String>,
    pub integration_public: bool,
    pub integration_require_code_grant: bool,
    pub bot_public: Option<bool>,
    pub bot_require_code_grant: Option<bool>,
    pub bot_disabled: bool,
    pub bot_quarantined: bool,
    pub approximate_guild_count: Option<u64>,
    pub approximate_user_install_count: u64,
    pub internal_guild_restriction: u64,
    pub terms_of_service_url: Option<String>,
    pub privacy_policy_url: Option<String>,
    pub role_connections_verification_url: Option<String>,
    pub interactions_endpoint_url: String,
    pub interactions_version: u64,
    pub interactions_event_types: Vec<String>,
    pub event_webhooks_status: Option<u64>,
    pub event_webhooks_url: Option<String>,
    pub event_webhooks_types: Option<Vec<String>>,
    pub explicit_content_filter: u64,
    pub tags: Option<Vec<String>>,
    pub install_params: Option<ApplicationInstallParams>,
    pub custom_install_url: Option<String>,
    pub integration_types_config: Option<Vec<ApplicationIntegrationTypeConfig>>,
    pub is_verified: bool,
    pub verification_state: Option<u64>,
    pub store_application_state: Option<u64>,
    pub rpc_application_state: Option<u64>,
    pub creator_monetization_state: Option<u64>,
    pub is_discoverable: bool,
    pub discoverability_state: Option<u64>,
    pub discovery_eligibility_flags: Option<u64>,
    pub is_monetized: bool,
    pub storefront_available: bool,
    pub monetization_state: Option<u64>,
    pub monetization_eligibility_flags: Option<u64>,
    pub max_participants: Option<u64>,
    pub embedded_activity_config: Option<EmbeddedActivityConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationExecutable {
    pub os: String,
    pub name: String,
    pub is_launcher: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSku {
    pub id: Option<String>,
    pub sku: Option<String>,
    pub distributor: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub owner_user_id: u64,
    pub members: Option<Vec<TeamMember>>,
    pub payout_account_status: Option<u64>,
    pub stripe_connect_account_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TeamMember {
    pub user: User,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub team_id: u64,
    pub membership_state: u64,
    pub role: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Company {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationInstallParams {
    pub scopes: Vec<String>,
    pub permissions: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationIntegrationTypeConfig {
    pub oauth2_install_params: Option<ApplicationInstallParams>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbeddedActivityConfig {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub application_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub activity_preview_video_asset_id: u64,
    pub supported_platforms: Vec<String>,
    pub default_orientation_lock_state: u64,
    pub tablet_default_orientation_lock_state: u64,
    pub requires_age_gate: bool,
    pub premium_tier_requirement: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub free_period_starts_at: Option<DateTime<Utc>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub free_period_ends_at: Option<DateTime<Utc>>,
    pub client_platform_config: Option<EmbeddedActivityPlatformConfig>,
    pub shelf_rank: u64,
    pub has_csp_exception: bool,
    pub displays_advertisements: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbeddedActivityPlatformConfig {
    pub label_type: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub label_until: Option<DateTime<Utc>>,
    pub release_phase: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Embed {
    pub title: Option<String>,
    #[serde(rename = "type")]
    pub embed_type: String,
    pub description: Option<String>,
    pub url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub timestamp: Option<DateTime<Utc>>,
    pub color: Option<u64>,
    pub footer: Option<EmbedFooter>,
    pub image: Option<EmbedImage>,
    pub thumbnail: Option<EmbedThumbnail>,
    pub video: Option<EmbedVideo>,
    pub provider: Option<EmbedProvider>,
    pub author: Option<EmbedAuthor>,
    pub fields: Option<Vec<EmbedField>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub reference_id: Option<u64>,
    pub flags: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedFooter {
    pub text: String,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedImage {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedThumbnail {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
//noinspection DuplicatedCode
pub struct EmbedVideo {
    pub url: String,
    pub proxy_url: Option<String>,
    pub height: Option<u64>,
    pub width: Option<u64>,
    pub flags: Option<u64>,
    pub placeholder_version: Option<u64>,
    pub placeholder: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedProvider {
    pub name: Option<String>,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedAuthor {
    pub name: String,
    pub url: Option<String>,
    pub icon_url: Option<String>,
    pub proxy_icon_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct EmbedField {
    pub name: String,
    pub value: String,
    pub inline: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Reaction {
    pub count: u64,
    pub count_details: ReactionCountDetails,
    pub me: bool,
    pub me_burst: bool,
    pub emoji: Emoji,
    pub burst_colors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReactionCountDetails {
    pub normal: u64,
    pub burst: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageActivity {
    #[serde(rename = "type")]
    pub activity_type: u8,
    pub session_id: String,
    pub party_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageReference {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_id: Option<u64>, // non existant whet a thread is created (wtf ?)
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
    pub content: String,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub timestamp: DateTime<Utc>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub edited_timestamp: Option<DateTime<Utc>>,
    pub mentions: Vec<User>,
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub mention_roles: Vec<u64>,
    pub attachments: Vec<Attachment>,
    pub embeds: Vec<Embed>,
    #[serde(rename = "type")]
    pub message_type: u8,
    pub flags: u64,
    pub components: Option<Vec<MessageComponent>>,
    pub sticker_items: Option<Vec<StickerItem>>,
    pub soundboard_sounds: Option<Vec<SoundboardSound>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageComponent {
    #[serde(rename = "type")]
    pub component_type: u8,
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

#[derive(Debug, Deserialize, Clone)]
pub struct SelectOption {
    pub label: String,
    pub value: String,
    pub description: Option<String>,
    pub emoji: Option<Emoji>,
    pub default: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StickerItem {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub format_type: u8,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SoundboardSound {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sound_id: u64,
    pub name: String,
    pub volume: f64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub available: bool,
    pub user: Option<User>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageCall {
    #[serde(deserialize_with = "deserialize_string_to_vec_u64")]
    pub participants: Vec<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub ended_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageInteraction {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageInteractionMetadata {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub interaction_type: u8,
    pub name: Option<String>,
    pub command_type: Option<u8>,
    pub ephemerality_reason: Option<u8>,
    pub user: User,
    pub authorizing_integration_owners: Option<HashMap<u64, String>>,
    pub original_response_message_id: Option<u64>,
    pub interacted_message_id: Option<u64>,
    pub triggering_interaction_metadata: Option<Box<MessageInteractionMetadata>>,
    pub target_user: Option<User>,
    pub target_message_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessageRoleSubscription {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub role_subscription_listing_id: u64,
    pub tier_name: String,
    pub total_months_subscribed: u64,
    pub is_renewal: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct MessagePurchaseNotification {
    #[serde(rename = "type")]
    pub purchase_type: u8,
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
pub struct Sticker {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub pack_id: Option<u64>,
    pub name: String,
    pub description: Option<String>,
    pub tags: String,
    pub sticker_type: u8,
    pub format_type: u8,
    pub available: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub user: Option<User>,
    pub sort_value: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Poll {
    pub question: PollMedia,
    pub answers: Vec<PollAnswer>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub expiry: Option<DateTime<Utc>>,
    pub allow_multiselect: bool,
    pub layout_type: u8,
    pub results: Option<PollResults>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollMedia {
    pub text: String,
    pub emoji: Option<Emoji>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollAnswer {
    pub answer_id: u64,
    pub poll_media: PollMedia,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollResults {
    pub is_finalized: bool,
    pub answer_counts: Vec<PollAnswerCount>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PollAnswerCount {
    pub id: u64,
    pub count: u64,
    pub me_voted: bool,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Potion {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub used_by: u64,
    pub r#type: u8,
    pub emoji: Vec<Emoji>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Channel {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub channel_type: u8,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub position: Option<u64>,
    pub permission_overwrites: Option<Vec<Overwrite>>,
    pub name: Option<String>,
    pub topic: Option<String>,
    pub nsfw: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub last_message_id: Option<u64>,
    pub bitrate: Option<u32>,
    pub user_limit: Option<u16>,
    pub rate_limit_per_user: Option<u32>,
    pub recipients: Option<Vec<User>>,
    pub icon: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub owner_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    pub managed: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub parent_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub last_pin_timestamp: Option<DateTime<Utc>>,
    pub rtc_region: Option<String>,
    pub video_quality_mode: Option<u8>,
    pub message_count: Option<u32>,
    pub member_count: Option<u32>,
    pub thread_metadata: Option<ThreadMetadata>,
    pub member: Option<ThreadMember>,
    pub default_auto_archive_duration: Option<u32>,
    pub permissions: Option<String>,
    pub flags: Option<u64>,
    pub total_message_sent: Option<u32>,
    pub available_tags: Option<Vec<Tag>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub applied_tags: Option<Vec<u64>>,
    pub default_reaction_emoji: Option<DefaultReaction>,
    pub default_thread_rate_limit_per_user: Option<u32>,
    pub default_sort_order: Option<u8>,
    pub default_forum_layout: Option<u8>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Overwrite {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(rename = "type")]
    pub overwrite_type: u8,
    pub allow: String,
    pub deny: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadMetadata {
    pub archived: bool,
    pub auto_archive_duration: u32,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub archive_timestamp: DateTime<Utc>,
    pub locked: bool,
    pub invitable: Option<bool>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub create_timestamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThreadMember {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    pub join_timestamp: DateTime<Utc>,
    pub flags: u64,
    pub member: Option<Member>
}

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub moderated: bool,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DefaultReaction {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub emoji_id: Option<u64>,
    pub emoji_name: Option<String>,
}