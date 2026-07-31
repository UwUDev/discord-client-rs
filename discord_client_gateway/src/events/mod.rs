use crate::events::gateway::GatewayPayload;
use crate::events::structs::application::*;
use crate::events::structs::call::*;
use crate::events::structs::channel::pin::*;
use crate::events::structs::channel::recipient::*;
use crate::events::structs::channel::stage::*;
use crate::events::structs::channel::summary::*;
use crate::events::structs::channel::thread::*;
use crate::events::structs::channel::typing::*;
use crate::events::structs::channel::voice::*;
use crate::events::structs::channel::webhook::WebhooksUpdateEvent;
use crate::events::structs::channel::*;
use crate::events::structs::extra::*;
use crate::events::structs::gateway::*;
use crate::events::structs::guild::ack::*;
use crate::events::structs::guild::automod::*;
use crate::events::structs::guild::ban::*;
use crate::events::structs::guild::emoji::*;
use crate::events::structs::guild::integration::*;
use crate::events::structs::guild::join_request::*;
use crate::events::structs::guild::role::*;
use crate::events::structs::guild::schedule_event::*;
use crate::events::structs::guild::soundboard::*;
use crate::events::structs::guild::sticker::*;
use crate::events::structs::guild::unread::*;
use crate::events::structs::guild::*;
use crate::events::structs::message::mention::RecentMentionDeleteEvent;
use crate::events::structs::message::poll::*;
use crate::events::structs::message::reaction::*;
use crate::events::structs::message::*;
use crate::events::structs::misc::*;
use crate::events::structs::notifications::GenericPushNotificationSentEvent;
use crate::events::structs::presence::*;
use crate::events::structs::ready::*;
use crate::events::structs::requested::*;
use crate::events::structs::stream::*;
use crate::events::structs::user::direct_message::DirectMessageSettingsUpsellShowEvent;
use crate::events::structs::user::note::UserNoteUpdateEvent;
use crate::events::structs::user::relationship::*;
use crate::events::structs::user::*;
use crate::events::structs::*;

pub(crate) mod deserializer;
pub mod structs;

macro_rules! define_events {
    (
        dispatch op $dispatch_op:expr, {
            $( $variant:ident { t: $t:expr, type: $event_struct:ty } ),+ $(,)?
        }
        $(
            , non_dispatch op $nd_op:expr, {
                $( $nd_variant:ident { t: $nd_t:expr, type: $nd_struct:ty } ),+ $(,)?
            }
        )*
    ) => {
        #[derive(Debug, Clone)]
        pub enum Event {
            // Dispatch events
            $(
                $variant($event_struct),
            )+

            // Non-dispatch events
            $(
                $(
                    $nd_variant($nd_struct),
                )+
            )*

            ParseError(ParseErrorEvent),
            Unknown(UnknownEvent),
        }

        impl Event {
            pub fn event_name(&self) -> &str {
                match self {
                    $(
                        Event::$variant(_) => $t,
                    )+
                    $(
                        $(
                            Event::$nd_variant(_) => $nd_t,
                        )+
                    )*
                    Event::ParseError(e) => &e.event_type,
                    Event::Unknown(unknown) => &unknown.r#type,
                }
            }
        }

        impl std::fmt::Display for Event {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                        Event::$variant(_) => write!(f, "{}", $t),
                    )+
                    $(
                        $(
                            Event::$nd_variant(_) => write!(f, "{}", $nd_t),
                        )+
                    )*
                    Event::ParseError(e) => write!(f, "ParseError ({}): {} at '{}'", e.event_type, e.error, e.path),
                    Event::Unknown(unknown) => write!(f, "Unknown ({}): {}", unknown.op, unknown.r#type),
                }
            }
        }

        pub fn parse_gateway_payload(payload: GatewayPayload) -> Result<Event, serde_json::Error> {
            match payload.op {
                // Dispatch events
                $dispatch_op => match payload.t.as_deref() {
                    $(
                        Some($t) => {
                            let result: Result<$event_struct, _> = serde_path_to_error::deserialize(&payload.d);

                            match result {
                                Ok(event) => Ok(Event::$variant(event)),
                                Err(err) => {
                                    Ok(Event::ParseError(ParseErrorEvent {
                                        event_type: $t.to_string(),
                                        op: payload.op,
                                        error: err.inner().to_string(),
                                        path: err.path().to_string(),
                                        raw: payload.d,
                                    }))
                                }
                            }
                        },
                    )+
                    Some(other) => Ok(Event::Unknown(UnknownEvent {
                        r#type: other.to_string(),
                        data: payload.d,
                        op: payload.op,
                    })),
                    None => Err(serde::de::Error::custom("Dispatch event missing 't' field")),
                },

                // Non-dispatch events
                $(
                    $nd_op => match serde_json::from_value(payload.d.clone()) {
                        $(
                            Ok(data) => Ok(Event::$nd_variant(data)),
                        )+
                        Err(_) => Ok(Event::Unknown(UnknownEvent {
                            r#type: "UNKNOWN_NON_DISPATCH".to_string(),
                            data: payload.d,
                            op: payload.op,
                        })),
                    },
                )*

                // Unknown opcodes
                _ => Ok(Event::Unknown(UnknownEvent {
                    r#type: "UNKNOWN_OP".to_string(),
                    data: payload.d,
                    op: payload.op,
                })),
            }
        }
    };
}

define_events! {
    dispatch op 0, {
        AuthSessionChange { t: "AUTH_SESSION_CHANGE", type: AuthSessionChangeEvent },
        AutoModMentionRaidDetection { t: "AUTO_MODERATION_MENTION_RAID_DETECTION", type: AutoModMentionRaidDetectionEvent },
        CallCreate { t: "CALL_CREATE", type: CallCreateEvent },
        CallDelete { t: "CALL_DELETE", type: CallDeleteEvent },
        CallUpdate { t: "CALL_UPDATE", type: CallUpdateEvent },
        ChannelCreate { t: "CHANNEL_CREATE", type: ChannelCreateEvent },
        ChannelDelete { t: "CHANNEL_DELETE", type: ChannelDeleteEvent },
        ChannelPinsAck { t: "CHANNEL_PINS_ACK", type: ChannelPinsAckEvent },
        ChannelPinsUpdate { t: "CHANNEL_PINS_UPDATE", type: ChannelPinsUpdateEvent },
        ChannelRecipientAdd { t: "CHANNEL_RECIPIENT_ADD", type: ChannelRecipientAddEvent },
        ChannelRecipientRemove { t: "CHANNEL_RECIPIENT_REMOVE", type: ChannelRecipientRemoveEvent },
        ChannelStatuses { t: "CHANNEL_STATUSES", type: ChannelStatusesEvent },
        ChannelUnreadUpdate { t: "CHANNEL_UNREAD_UPDATE", type: ChannelUnreadUpdateEvent },
        ChannelUpdate { t: "CHANNEL_UPDATE", type: ChannelUpdateEvent },
        ContentInventoryInboxStale { t: "CONTENT_INVENTORY_INBOX_STALE", type: ContentInventoryInboxStaleEvent },
        ConversationSummaryUpdate { t: "CONVERSATION_SUMMARY_UPDATE", type: ConversationSummaryUpdateEvent },
        DirectMessageSettingsUpsellShowEvent { t: "DM_SETTINGS_UPSELL_SHOW", type: DirectMessageSettingsUpsellShowEvent },
        GuildAuditLogEntryCreate { t: "GUILD_AUDIT_LOG_ENTRY_CREATE", type: GuildAuditLogEntryCreateEvent },
        GenericPushNotificationSent { t: "GENERIC_PUSH_NOTIFICATION_SENT", type: GenericPushNotificationSentEvent },
        GuildAppliedBoostsUpdate { t: "GUILD_APPLIED_BOOSTS_UPDATE", type: GuildAppliedBoostsUpdateEvent },
        GuildBanAdd { t: "GUILD_BAN_ADD", type: GuildBanAddEvent },
        GuildBanRemove { t: "GUILD_BAN_REMOVE", type: GuildBanRemoveEvent },
        GuildCreate { t: "GUILD_CREATE", type: GuildCreateEvent },
        GuildDelete { t: "GUILD_DELETE", type: GuildDeleteEvent },
        GuildEmojisUpdate { t: "GUILD_EMOJIS_UPDATE", type: GuildEmojisUpdateEvent },
        GuildFeatureAck { t: "GUILD_FEATURE_ACK", type: GuildFeatureAckEvent },
        GuildIntegrationCreate { t: "INTEGRATION_CREATE", type: IntegrationCreateEvent },
        GuildIntegrationDelete { t: "INTEGRATION_DELETE", type: IntegrationDeleteEvent },
        GuildIntegrationsUpdate { t: "GUILD_INTEGRATIONS_UPDATE", type: GuildIntegrationsUpdateEvent },
        GuildIntegrationUpdate { t: "INTEGRATION_UPDATE", type: IntegrationUpdateEvent },
        GuildJoinRequestCreate { t: "GUILD_JOIN_REQUEST_CREATE", type: GuildJoinRequestCreateEvent },
        GuildJoinRequestDelete { t: "GUILD_JOIN_REQUEST_DELETE", type: GuildJoinRequestDeleteEvent },
        GuildJoinRequestUpdate { t: "GUILD_JOIN_REQUEST_UPDATE", type: GuildJoinRequestUpdateEvent },
        GuildMemberAdd { t: "GUILD_MEMBER_ADD", type: GuildMemberAddEvent },
        GuildMemberRemove { t: "GUILD_MEMBER_REMOVE", type: GuildMemberRemoveEvent },
        GuildMembersChunk { t: "GUILD_MEMBERS_CHUNK", type: GuildMembersChunkEvent },
        GuildMemberUpdate { t: "GUILD_MEMBER_UPDATE", type: GuildMemberUpdateEvent },
        GuildRoleCreate { t: "GUILD_ROLE_CREATE", type: GuildRoleCreateEvent },
        GuildRoleDelete { t: "GUILD_ROLE_DELETE", type: GuildRoleDeleteEvent },
        GuildRoleUpdate { t: "GUILD_ROLE_UPDATE", type: GuildRoleUpdateEvent },
        GuildScheduledEventCreate { t: "GUILD_SCHEDULED_EVENT_CREATE", type: GuildScheduledEventCreateEvent },
        GuildScheduledEventDelete { t: "GUILD_SCHEDULED_EVENT_DELETE", type: GuildScheduledEventDeleteEvent },
        GuildScheduledEventExceptionCreate { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_CREATE", type: GuildScheduledEventExceptionCreateEvent },
        GuildScheduledEventExceptionDelete { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_DELETE", type: GuildScheduledEventExceptionDeleteEvent },
        GuildScheduledEventExceptionsDelete { t: "GUILD_SCHEDULED_EVENT_EXCEPTIONS_DELETE", type: GuildScheduledEventExceptionsDeleteEvent },
        GuildScheduledEventExceptionUpdate { t: "GUILD_SCHEDULED_EVENT_EXCEPTION_UPDATE", type: GuildScheduledEventExceptionUpdateEvent },
        GuildScheduledEventUpdate { t: "GUILD_SCHEDULED_EVENT_UPDATE", type: GuildScheduledEventUpdateEvent },
        GuildScheduledEventUserAdd { t: "GUILD_SCHEDULED_EVENT_USER_ADD", type: GuildScheduledEventUserAddEvent },
        GuildScheduledEventUserRemove { t: "GUILD_SCHEDULED_EVENT_USER_REMOVE", type: GuildScheduledEventUserRemoveEvent },
        GuildSoundboardSoundCreate { t: "GUILD_SOUNDBOARD_SOUND_CREATE", type: GuildSoundboardSoundCreateEvent },
        GuildSoundboardSoundDelete { t: "GUILD_SOUNDBOARD_SOUND_DELETE", type: GuildSoundboardSoundDeleteEvent },
        GuildSoundboardSoundUpdate { t: "GUILD_SOUNDBOARD_SOUND_UPDATE", type: GuildSoundboardSoundUpdateEvent },
        GuildStickersUpdate { t: "GUILD_STICKERS_UPDATE", type: GuildStickersUpdateEvent },
        GuildUpdate { t: "GUILD_UPDATE", type: GuildUpdateEvent },
        LastMessages { t: "LAST_MESSAGES", type: LastMessagesEvent },
        MessageAck { t: "MESSAGE_ACK", type: MessageAckEvent },
        MessageCreate { t: "MESSAGE_CREATE", type: MessageCreateEvent },
        MessageDelete { t: "MESSAGE_DELETE", type: MessageDeleteEvent },
        MessageDeleteBulk { t: "MESSAGE_DELETE_BULK", type: MessageDeleteBulkEvent },
        RecentMentionDelete { t: "RECENT_MENTION_DELETE", type: RecentMentionDeleteEvent },
        MessagePollVoteAdd { t: "MESSAGE_POLL_VOTE_ADD", type: MessagePollVoteAddEvent },
        MessagePollVoteRemove { t: "MESSAGE_POLL_VOTE_REMOVE", type: MessagePollVoteRemoveEvent },
        MessageReactionAdd { t: "MESSAGE_REACTION_ADD", type: MessageReactionAddEvent },
        MessageReactionAddMany { t: "MESSAGE_REACTION_ADD_MANY", type: MessageReactionAddManyEvent },
        MessageReactionRemove { t: "MESSAGE_REACTION_REMOVE", type: MessageReactionRemoveEvent },
        MessageReactionRemoveAll { t: "MESSAGE_REACTION_REMOVE_ALL", type: MessageReactionRemoveAllEvent },
        MessageReactionRemoveEmoji { t: "MESSAGE_REACTION_REMOVE_EMOJI", type: MessageReactionRemoveEmojiEvent },
        MessageUpdate { t: "MESSAGE_UPDATE", type: MessageUpdateEvent },
        PassiveUpdateV2 { t: "PASSIVE_UPDATE_V2", type: PassiveUpdateV2Event },
        PresenceUpdate { t: "PRESENCE_UPDATE", type: PresenceUpdateEvent },
        Ready { t: "READY", type: ReadyEvent },
        ReadySupplemental { t: "READY_SUPPLEMENTAL", type: ReadySupplementalEvent },
        RelationshipAdd { t: "RELATIONSHIP_ADD", type: RelationshipAddEvent },
        RelationshipRemove { t: "RELATIONSHIP_REMOVE", type: RelationshipRemoveEvent },
        RelationshipUpdate { t: "RELATIONSHIP_UPDATE", type: RelationshipUpdateEvent },
        RemoteCommand { t: "REMOTE_COMMAND", type: RemoteCommandEvent },
        Resumed { t: "RESUMED", type: ResumedEvent },
        SessionReplace { t: "SESSIONS_REPLACE", type: SessionsReplaceEvent },
        SoundboardSounds { t: "SOUNDBOARD_SOUNDS", type: SoundboardSoundsEvent },
        StageInstanceCreate { t: "STAGE_INSTANCE_CREATE", type: StageInstanceCreateEvent },
        StageInstanceDelete { t: "STAGE_INSTANCE_DELETE", type: StageInstanceDeleteEvent },
        StageInstanceUpdate { t: "STAGE_INSTANCE_UPDATE", type: StageInstanceUpdateEvent },
        ThreadCreate { t: "THREAD_CREATE", type: ThreadCreateEvent },
        ThreadDelete { t: "THREAD_DELETE", type: ThreadDeleteEvent },
        ThreadListSync { t: "THREAD_LIST_SYNC", type: ThreadListSyncEvent },
        ThreadMembersUpdate { t: "THREAD_MEMBERS_UPDATE", type: ThreadMembersUpdateEvent },
        ThreadMemberUpdate { t: "THREAD_MEMBER_UPDATE", type: ThreadMemberUpdateEvent },
        ThreadUpdate { t: "THREAD_UPDATE", type: ThreadUpdateEvent },
        TypingStart { t: "TYPING_START", type: TypingStartEvent },
        UserConnectionsUpdate { t: "USER_CONNECTIONS_UPDATE", type: UserConnectionsUpdateEvent },
        UserGuildSettingsUpdate { t: "USER_GUILD_SETTINGS_UPDATE", type: UserGuildSettingsUpdateEvent },
        UserNoteUpdateEvent { t: "USER_NOTE_UPDATE", type: UserNoteUpdateEvent },
        UserSettingsProtoUpdate { t: "USER_SETTINGS_PROTO_UPDATE", type: UserSettingsProtoUpdateEvent },
        VoiceChannelStatusUpdate { t: "VOICE_CHANNEL_STATUS_UPDATE", type: VoiceChannelStatusUpdateEvent },
        VoiceChannelStartTimeUpdate { t: "VOICE_CHANNEL_START_TIME_UPDATE", type: VoiceChannelStartTimeUpdateEvent },
        VoiceStateUpdate { t: "VOICE_STATE_UPDATE", type: VoiceStateUpdateEvent },
        VoiceStateUpdateBatch { t: "VOICE_STATE_UPDATE_BATCH", type: VoiceStateUpdateBatchEvent },
        WebhookUpdate { t: "WEBHOOKS_UPDATE", type: WebhooksUpdateEvent },
        ApplicationCommandPermissionsUpdate { t: "APPLICATION_COMMAND_PERMISSIONS_UPDATE", type: ApplicationCommandPermissionsUpdateEvent },
        VoiceServerUpdate { t: "VOICE_SERVER_UPDATE", type: VoiceServerUpdateEvent },
        AuthenticatorCreate { t: "AUTHENTICATOR_CREATE", type: AuthenticatorCreateEvent },
        AuthenticatorUpdate { t: "AUTHENTICATOR_UPDATE", type: AuthenticatorUpdateEvent },
        AuthenticatorDelete { t: "AUTHENTICATOR_DELETE", type: AuthenticatorDeleteEvent },
        OAuth2TokenRevoke { t: "OAUTH2_TOKEN_REVOKE", type: OAuth2TokenRevokeEvent },
        StreamCreate { t: "STREAM_CREATE", type: StreamCreateEvent },
        StreamUpdate { t: "STREAM_UPDATE", type: StreamUpdateEvent },
        StreamServerUpdate { t: "STREAM_SERVER_UPDATE", type: StreamServerUpdateEvent },
        StreamDelete { t: "STREAM_DELETE", type: StreamDeleteEvent },
        UserApplicationUpdate { t: "USER_APPLICATION_UPDATE", type: UserApplicationUpdateEvent },
        UserApplicationRemove { t: "USER_APPLICATION_REMOVE", type: UserApplicationRemoveEvent },
        UserMergeOperationCompleted { t: "USER_MERGE_OPERATION_COMPLETED", type: UserMergeOperationCompletedEvent },
        UserRequiredActionUpdate { t: "USER_REQUIRED_ACTION_UPDATE", type: UserRequiredActionUpdateEvent },
        UserSettingsUpdate { t: "USER_SETTINGS_UPDATE", type: UserSettingsUpdateEvent },
        GameRelationshipAdd { t: "GAME_RELATIONSHIP_ADD", type: GameRelationshipAddEvent },
        GameRelationshipRemove { t: "GAME_RELATIONSHIP_REMOVE", type: GameRelationshipRemoveEvent },
        FriendSuggestionCreate { t: "FRIEND_SUGGESTION_CREATE", type: FriendSuggestionCreateEvent },
        FriendSuggestionDelete { t: "FRIEND_SUGGESTION_DELETE", type: FriendSuggestionDeleteEvent },
        RateLimited { t: "RATE_LIMITED", type: RateLimitedEvent },
        ActivityInviteCreate { t: "ACTIVITY_INVITE_CREATE", type: ActivityInviteCreateEvent },
        BillingPopupBridgeCallback { t: "BILLING_POPUP_BRIDGE_CALLBACK", type: BillingPopupBridgeCallbackEvent },
        ChannelSync { t: "CHANNEL_SYNC", type: ChannelSyncEvent },
        ChannelUpdatePartial { t: "CHANNEL_UPDATE_PARTIAL", type: ChannelUpdatePartialEvent },
        ChannelInfo { t: "CHANNEL_INFO", type: ChannelInfoEvent },
        ChannelMemberCountUpdate { t: "CHANNEL_MEMBER_COUNT_UPDATE", type: ChannelMemberCountUpdateEvent },
        ConsoleCommandUpdate { t: "CONSOLE_COMMAND_UPDATE", type: ConsoleCommandUpdateEvent },
        CreatorMonetizationRestrictionsUpdate { t: "CREATOR_MONETIZATION_RESTRICTIONS_UPDATE", type: CreatorMonetizationRestrictionsUpdateEvent },
        DeletedEntityIds { t: "DELETED_ENTITY_IDS", type: DeletedEntityIdsEvent },
        EmbeddedActivityUpdateV2 { t: "EMBEDDED_ACTIVITY_UPDATE_V2", type: EmbeddedActivityUpdateV2Event },
        EntitlementCreate { t: "ENTITLEMENT_CREATE", type: EntitlementCreateEvent },
        EntitlementUpdate { t: "ENTITLEMENT_UPDATE", type: EntitlementUpdateEvent },
        EntitlementDelete { t: "ENTITLEMENT_DELETE", type: EntitlementDeleteEvent },
        ExperimentSessionOverrideCreate { t: "EXPERIMENT_SESSION_OVERRIDE_CREATE", type: ExperimentSessionOverrideCreateEvent },
        ExperimentSessionOverrideDelete { t: "EXPERIMENT_SESSION_OVERRIDE_DELETE", type: ExperimentSessionOverrideDeleteEvent },
        GameServerCreate { t: "GAME_SERVER_CREATE", type: GameServerCreateEvent },
        GameServerUpdate { t: "GAME_SERVER_UPDATE", type: GameServerUpdateEvent },
        GameServerDelete { t: "GAME_SERVER_DELETE", type: GameServerDeleteEvent },
        GiftCodeCreate { t: "GIFT_CODE_CREATE", type: GiftCodeCreateEvent },
        GiftCodeUpdate { t: "GIFT_CODE_UPDATE", type: GiftCodeUpdateEvent },
        GuildApplicationCommandIndexUpdate { t: "GUILD_APPLICATION_COMMAND_INDEX_UPDATE", type: GuildApplicationCommandIndexUpdateEvent },
        GuildDirectoryEntryCreate { t: "GUILD_DIRECTORY_ENTRY_CREATE", type: GuildDirectoryEntryCreateEvent },
        GuildDirectoryEntryUpdate { t: "GUILD_DIRECTORY_ENTRY_UPDATE", type: GuildDirectoryEntryUpdateEvent },
        GuildDirectoryEntryDelete { t: "GUILD_DIRECTORY_ENTRY_DELETE", type: GuildDirectoryEntryDeleteEvent },
        GuildOfficialGameApplicationsUpdate { t: "GUILD_OFFICIAL_GAME_APPLICATIONS_UPDATE", type: GuildOfficialGameApplicationsUpdateEvent },
        GuildPowerupEntitlementsCreate { t: "GUILD_POWERUP_ENTITLEMENTS_CREATE", type: GuildPowerupEntitlementsCreateEvent },
        GuildPowerupEntitlementsDelete { t: "GUILD_POWERUP_ENTITLEMENTS_DELETE", type: GuildPowerupEntitlementsDeleteEvent },
        GuildSoundboardSoundsUpdate { t: "GUILD_SOUNDBOARD_SOUNDS_UPDATE", type: GuildSoundboardSoundsUpdateEvent },
        InteractionCreate { t: "INTERACTION_CREATE", type: InteractionCreateEvent },
        InteractionFailure { t: "INTERACTION_FAILURE", type: InteractionFailureEvent },
        InteractionSuccess { t: "INTERACTION_SUCCESS", type: InteractionSuccessEvent },
        ApplicationCommandAutocompleteResponse { t: "APPLICATION_COMMAND_AUTOCOMPLETE_RESPONSE", type: ApplicationCommandAutocompleteResponseEvent },
        InteractionModalCreate { t: "INTERACTION_MODAL_CREATE", type: InteractionModalCreateEvent },
        InteractionIFrameModalCreate { t: "INTERACTION_IFRAME_MODAL_CREATE", type: InteractionIFrameModalCreateEvent },
        SocialLayerSkuPurchaseEligibilityResponse { t: "SOCIAL_LAYER_SKU_PURCHASE_ELIGIBILITY_RESPONSE", type: SocialLayerSkuPurchaseEligibilityResponseEvent },
        ReactionNotificationSent { t: "REACTION_NOTIFICATION_SENT", type: ReactionNotificationSentEvent },
        NotificationCenterItemCreate { t: "NOTIFICATION_CENTER_ITEM_CREATE", type: NotificationCenterItemCreateEvent },
        NotificationCenterItemDelete { t: "NOTIFICATION_CENTER_ITEM_DELETE", type: NotificationCenterItemDeleteEvent },
        NotificationCenterItemsAck { t: "NOTIFICATION_CENTER_ITEMS_ACK", type: NotificationCenterItemsAckEvent },
        NotificationCenterItemCompleted { t: "NOTIFICATION_CENTER_ITEM_COMPLETED", type: NotificationCenterItemCompletedEvent },
        NotificationSettingsUpdate { t: "NOTIFICATION_SETTINGS_UPDATE", type: NotificationSettingsUpdateEvent },
        OAuth2TokenCreate { t: "OAUTH2_TOKEN_CREATE", type: OAuth2TokenCreateEvent },
        OAuth2TokenDelete { t: "OAUTH2_TOKEN_DELETE", type: OAuth2TokenDeleteEvent },
        PaymentUpdate { t: "PAYMENT_UPDATE", type: PaymentUpdateEvent },
        QuestsUserStatusUpdate { t: "QUESTS_USER_STATUS_UPDATE", type: QuestsUserStatusUpdateEvent },
        QuestsUserCompletionUpdate { t: "QUESTS_USER_COMPLETION_UPDATE", type: QuestsUserCompletionUpdateEvent },
        GameInviteCreate { t: "GAME_INVITE_CREATE", type: GameInviteCreateEvent },
        GameInviteDelete { t: "GAME_INVITE_DELETE", type: GameInviteDeleteEvent },
        GameInviteDeleteMany { t: "GAME_INVITE_DELETE_MANY", type: GameInviteDeleteManyEvent },
        LobbyCreate { t: "LOBBY_CREATE", type: LobbyCreateEvent },
        LobbyUpdate { t: "LOBBY_UPDATE", type: LobbyUpdateEvent },
        LobbyDelete { t: "LOBBY_DELETE", type: LobbyDeleteEvent },
        LobbyMemberAdd { t: "LOBBY_MEMBER_ADD", type: LobbyMemberAddEvent },
        LobbyMemberUpdate { t: "LOBBY_MEMBER_UPDATE", type: LobbyMemberUpdateEvent },
        LobbyMemberRemove { t: "LOBBY_MEMBER_REMOVE", type: LobbyMemberRemoveEvent },
        LobbyMessageCreate { t: "LOBBY_MESSAGE_CREATE", type: LobbyMessageCreateEvent },
        LobbyMessageUpdate { t: "LOBBY_MESSAGE_UPDATE", type: LobbyMessageUpdateEvent },
        LobbyMessageDelete { t: "LOBBY_MESSAGE_DELETE", type: LobbyMessageDeleteEvent },
        LobbyVoiceStateUpdate { t: "LOBBY_VOICE_STATE_UPDATE", type: LobbyVoiceStateUpdateEvent },
        LobbyVoiceServerUpdate { t: "LOBBY_VOICE_SERVER_UPDATE", type: LobbyVoiceServerUpdateEvent },
        PassiveUpdateV1 { t: "PASSIVE_UPDATE_V1", type: PassiveUpdateV1Event },
        SavedMessageCreate { t: "SAVED_MESSAGE_CREATE", type: SavedMessageCreateEvent },
        SavedMessageDelete { t: "SAVED_MESSAGE_DELETE", type: SavedMessageDeleteEvent },
        SpeedTestCreate { t: "SPEED_TEST_CREATE", type: SpeedTestCreateEvent },
        SpeedTestServerUpdate { t: "SPEED_TEST_SERVER_UPDATE", type: SpeedTestServerUpdateEvent },
        SpeedTestUpdate { t: "SPEED_TEST_UPDATE", type: SpeedTestUpdateEvent },
        SpeedTestDelete { t: "SPEED_TEST_DELETE", type: SpeedTestDeleteEvent },
        UserUpdate { t: "USER_UPDATE", type: UserUpdateEvent },
        UserApplicationIdentityUpdate { t: "USER_APPLICATION_IDENTITY_UPDATE", type: UserApplicationIdentityUpdateEvent },
        UserApplicationIdentityRemove { t: "USER_APPLICATION_IDENTITY_REMOVE", type: UserApplicationIdentityRemoveEvent },
        UserNonChannelAck { t: "USER_NON_CHANNEL_ACK", type: UserNonChannelAckEvent },
        UserPremiumGuildSubscriptionSlotCreate { t: "USER_PREMIUM_GUILD_SUBSCRIPTION_SLOT_CREATE", type: UserPremiumGuildSubscriptionSlotCreateEvent },
        UserPremiumGuildSubscriptionSlotUpdate { t: "USER_PREMIUM_GUILD_SUBSCRIPTION_SLOT_UPDATE", type: UserPremiumGuildSubscriptionSlotUpdateEvent },
        UserPremiumGuildSubscriptionSlotDelete { t: "USER_PREMIUM_GUILD_SUBSCRIPTION_SLOT_DELETE", type: UserPremiumGuildSubscriptionSlotDeleteEvent },
        AudioSettingsUpdate { t: "AUDIO_SETTINGS_UPDATE", type: AudioSettingsUpdateEvent },
        UserPaymentBrowserCheckoutDone { t: "USER_PAYMENT_BROWSER_CHECKOUT_DONE", type: UserPaymentBrowserCheckoutDoneEvent },
        UserPaymentClientAdd { t: "USER_PAYMENT_CLIENT_ADD", type: UserPaymentClientAddEvent },
        UserPaymentSourcesUpdate { t: "USER_PAYMENT_SOURCES_UPDATE", type: UserPaymentSourcesUpdateEvent },
        UserSubscriptionsUpdate { t: "USER_SUBSCRIPTIONS_UPDATE", type: UserSubscriptionsUpdateEvent },
        VoiceChannelEffectSend { t: "VOICE_CHANNEL_EFFECT_SEND", type: VoiceChannelEffectSendEvent },
        VirtualCurrencyBalanceUpdate { t: "VIRTUAL_CURRENCY_BALANCE_UPDATE", type: VirtualCurrencyBalanceUpdateEvent },
    },
    non_dispatch op 7, {
        GatewayReconnect { t: "RECONNECT", type: GatewayReconnectEvent }
    },
    non_dispatch op 11, {
        HeartbeatAck { t: "HEARTBEAT_ACK", type: HeartbeatAckEvent }
    },
    non_dispatch op 9, {
        InvalidSession { t: "INVALID_SESSION", type: InvalidSessionEvent }
    }
}
