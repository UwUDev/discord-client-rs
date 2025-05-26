use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GenericPushNotificationSentEvent {
    pub user_username: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
    pub user_discriminator: Option<String>,
    pub user_avatar: Option<String>,
    pub r#type: String,
    pub tracking_type: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub route: String,
    pub notification_type: String,
    pub notif_type_id: String,
    #[serde(rename = "deserialize_string_to_u64")]
    pub notif_instance_id: u64,
    pub message_type_: u8,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub message_id: u64,
    pub icon: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    pub deeplink: Option<String>,
    pub channel_type: u8,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub channel_id: u64,
    pub body: Option<String>,
}
