use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::activity::GameActivity;
use crate::structs::user::User;
use chrono::{DateTime, Utc};
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Clan {
    #[serde(default)]
    #[snowflake]
    pub id: Option<u64>,
    #[serde(default)]
    pub name: Option<String>,
    pub tag: Option<String>,
    pub icon_hash: Option<String>,
    pub member_count: Option<u32>,
    pub description: Option<String>,
    pub play_style: Option<u8>,
    pub search_terms: Option<Vec<String>>,
    pub game_application_ids: Option<Vec<u64>>,
    pub badge: Option<String>,
    pub badge_hash: Option<String>,
    pub badge_color_primary: Option<String>,
    pub badge_color_secondary: Option<String>,
    pub banner: Option<u8>,
    pub banner_hash: Option<String>,
    pub brand_color_primary: Option<String>,
    pub brand_color_secondary: Option<String>,
    pub wildcard_descriptors: Option<Vec<String>>,
    pub game_activity: Option<Vec<GameActivity>>,
    pub discovery_profile_features: Option<Vec<String>>,
}

#[discord_struct]
pub struct GuildJoinRequest {
    #[snowflake]
    pub id: u64,
    #[snowflake]
    pub join_request_id: u64,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub created_at: DateTime<Utc>,
    pub application_status: String,
    #[snowflake]
    pub guild_id: u64,
    #[serde(default)]
    pub form_responses: Option<Vec<MemberVerificationFormField>>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub last_seen: Option<DateTime<Utc>>,
    #[serde(default)]
    #[snowflake(no_created_at)]
    pub actioned_at: Option<u64>,
    #[serde(default)]
    pub actioned_by_user: Option<User>,
    #[serde(default)]
    pub rejection_reason: Option<String>,
    #[snowflake]
    pub user_id: u64,
    #[serde(default)]
    pub user: Option<User>,
    #[serde(default)]
    #[snowflake]
    pub interview_channel_id: Option<u64>,
}

#[discord_struct]
pub struct MemberVerificationFormField {
    pub field_type: String,
    pub label: String,
    #[serde(default)]
    pub choices: Option<Vec<String>>,
    #[serde(default)]
    pub values: Option<Vec<String>>,
    #[serde(default)]
    pub response: Option<serde_json::Value>,
    pub required: bool,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub automations: Option<Vec<String>>,
    #[serde(default)]
    pub placeholder: Option<String>,
}

#[discord_struct]
pub struct ClanBadge {
    pub tag: Option<String>,
    #[snowflake]
    pub identity_guild_id: Option<u64>,
    pub identity_enabled: Option<bool>,
    pub badge: Option<String>,
}

impl ClanBadge {
    pub fn get_image_url(&self, custom_size: Option<u16>) -> Option<String> {
        if self.badge.is_none() || self.identity_guild_id.is_none() {
            return None;
        }

        if let Some(size) = custom_size {
            Some(format!(
                "https://cdn.discordapp.com/clan-badges/{}/{}.png?size={}",
                self.identity_guild_id.unwrap(),
                self.clone().badge.unwrap(),
                size
            ))
        } else {
            Some(format!(
                "https://cdn.discordapp.com/clan-badges/{}/{}.png",
                self.identity_guild_id.unwrap(),
                self.clone().badge.unwrap()
            ))
        }
    }
}
