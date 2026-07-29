use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_option_string_to_vec_u64;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct RateLimitedEvent {
    #[serde(default)]
    pub opcode: Option<i64>,
    #[serde(default)]
    pub retry_after: Option<f64>,
    #[serde(default)]
    pub meta: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActivityInviteCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub author: Option<Value>,
    #[serde(default)]
    pub application: Option<Value>,
    #[serde(default)]
    pub activity: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BillingPopupBridgeCallbackEvent {
    #[serde(default)]
    pub payment_source_type: Option<i64>,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub query: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelSyncEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub channels: Option<Value>,
    #[serde(default)]
    pub integrity_check: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct ChannelUpdatePartialEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelInfoEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub channels: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub voice_start_time: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ChannelMemberCountUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub member_count: Option<i64>,
    #[serde(default)]
    pub presence_count: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConsoleCommandUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    pub result: Option<String>,
    #[serde(default)]
    pub error: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreatorMonetizationRestrictionsUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub restrictions: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeletedEntityIdsEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub stickers: Option<Vec<u64>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub roles: Option<Vec<u64>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub emojis: Option<Vec<u64>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub channels: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct EmbeddedActivityUpdateV2Event {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct EntitlementCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct EntitlementUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct EntitlementDeleteEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExperimentSessionOverrideCreateEvent {
    #[serde(default)]
    pub experiment_name: Option<String>,
    #[serde(default)]
    pub variant_id: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ExperimentSessionOverrideDeleteEvent {
    #[serde(default)]
    pub experiment_name: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameServerCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub game_server: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameServerUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub game_server: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameServerDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub game_server_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct GiftCodeCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct GiftCodeUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildApplicationCommandIndexUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct GuildDirectoryEntryCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct GuildDirectoryEntryUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildDirectoryEntryDeleteEvent {
    #[serde(default)]
    pub r#type: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub directory_channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub entity_id: Option<u64>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub primary_category_id: Option<i64>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub author_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildOfficialGameApplicationsUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub game_application_ids: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildPowerupEntitlementsCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub entitlements: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildPowerupEntitlementsDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub entitlements: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildSoundboardSoundsUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub soundboard_sounds: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionFailureEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    pub nonce: Option<String>,
    #[serde(default)]
    pub reason_code: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionSuccessEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationCommandAutocompleteResponseEvent {
    #[serde(default)]
    pub choices: Option<Value>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionModalCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub custom_id: Option<String>,
    #[serde(default)]
    pub application: Option<Value>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub components: Option<Value>,
    #[serde(default)]
    pub nonce: Option<String>,
    #[serde(default)]
    pub resolved: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InteractionIFrameModalCreateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    pub custom_id: Option<String>,
    #[serde(default)]
    pub application: Option<Value>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub iframe_path: Option<String>,
    #[serde(default)]
    pub modal_size: Option<i64>,
    #[serde(default)]
    pub nonce: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SocialLayerSkuPurchaseEligibilityResponseEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub interaction_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub sku_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub recipient_id: Option<u64>,
    #[serde(default)]
    pub eligible: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ReactionNotificationSentEvent {
    #[serde(default)]
    pub message: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub reactor_user_id: Option<u64>,
    #[serde(default)]
    pub emoji: Option<Value>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
    #[serde(default)]
    pub tracking_type: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct NotificationCenterItemCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationCenterItemDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationCenterItemsAckEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NotificationCenterItemCompletedEvent {
    #[serde(default)]
    pub item_enum: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct NotificationSettingsUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct OAuth2TokenCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OAuth2TokenDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct PaymentUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuestsUserStatusUpdateEvent {
    #[serde(default)]
    pub user_status: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct QuestsUserCompletionUpdateEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub quest_id: Option<u64>,
    #[serde(default)]
    pub completed_at: Option<String>,
    #[serde(default)]
    pub stream_progress_seconds: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct GameInviteCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameInviteDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub invite_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GameInviteDeleteManyEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub invite_ids: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct LobbyCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct LobbyUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyMemberAddEvent {
    #[serde(default)]
    pub member: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyMemberUpdateEvent {
    #[serde(default)]
    pub member: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyMemberRemoveEvent {
    #[serde(default)]
    pub member: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct LobbyMessageCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct LobbyMessageUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyMessageDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct LobbyVoiceStateUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LobbyVoiceServerUpdateEvent {
    #[serde(default)]
    pub token: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub lobby_id: Option<u64>,
    #[serde(default)]
    pub endpoint: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PassiveUpdateV1Event {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    pub channels: Option<Value>,
    #[serde(default)]
    pub voice_states: Option<Value>,
    #[serde(default)]
    pub members: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct SavedMessageCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SavedMessageDeleteEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub message_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct SpeedTestCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct SpeedTestServerUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct SpeedTestUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct SpeedTestDeleteEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserApplicationIdentityUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserApplicationIdentityRemoveEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserNonChannelAckEvent {
    #[serde(default)]
    pub ack_type: Option<i64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub resource_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub entity_id: Option<u64>,
    #[serde(default)]
    pub version: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserPremiumGuildSubscriptionSlotCreateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserPremiumGuildSubscriptionSlotUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserPremiumGuildSubscriptionSlotDeleteEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AudioSettingsUpdateEvent {
    #[serde(default)]
    pub user: Option<Value>,
    #[serde(default)]
    pub stream: Option<Value>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserPaymentBrowserCheckoutDoneEvent {
    #[serde(default)]
    pub load_id: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub sku_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub sku_subscription_plan_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserPaymentClientAddEvent {
    #[serde(default)]
    pub purchase_token_hash: Option<String>,
    #[serde(default)]
    pub expires_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserPaymentSourcesUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(transparent)]
pub struct UserSubscriptionsUpdateEvent {
    pub inner: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceChannelEffectSendEvent {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub user_id: Option<u64>,
    #[serde(default)]
    pub animation_type: Option<i64>,
    #[serde(default)]
    pub animation_id: Option<i64>,
    #[serde(default)]
    pub emoji: Option<Value>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub sound_id: Option<u64>,
    #[serde(default)]
    pub sound_volume: Option<f64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VirtualCurrencyBalanceUpdateEvent {
    #[serde(default)]
    pub balance: Option<i64>,
}
