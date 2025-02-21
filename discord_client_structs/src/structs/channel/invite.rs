use crate::structs::application::Application;
use crate::structs::channel::Channel;
use crate::structs::channel::*;
use crate::structs::guild::event::GuildScheduledEvent;
use crate::structs::user::{Member, User};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct Invite {
    pub code: String,
    pub r#type: u8,
    #[serde(flatten)]
    pub metadata: InviteMetadata,
    pub channel: Option<Channel>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub guild_id: Option<u64>,
    pub guild: Option<InviteGuild>,
    pub inviter: Option<User>,
    pub flags: Option<u64>,
    pub target_type: Option<u8>,
    pub target_user: Option<User>,
    pub target_application: Option<Application>,
    pub approximate_member_count: Option<u32>,
    pub approximate_presence_count: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    pub expires_at: Option<DateTime<Utc>>,
    pub stage_instance: Option<InviteStageInstance>,
    pub guild_scheduled_event: Option<GuildScheduledEvent>,
    pub new_member: Option<bool>,
    pub show_verification_form: Option<bool>,
    pub is_nickname_changeable: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct InviteStageInstance {
    pub members: Vec<Member>,
    pub participant_count: u32,
    pub speaker_count: u32,
    pub topic: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default, CreatedAt)]
#[builder(setter(into, strip_option), default)]
pub struct InviteGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub banner: Option<String>,
    pub splash: Option<String>,
    pub verification_level: u8,
    pub features: Vec<String>,
    pub vanity_url_code: Option<String>,
    pub premium_subscription_count: Option<u32>,
    pub nsfw: Option<bool>,
    pub nsfw_level: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct CreateChannelInvite {
    pub flags: Option<u64>,
    pub max_age: Option<u64>,
    pub max_uses: Option<u64>,
    pub temporary: Option<bool>,
    pub unique: Option<bool>,
    pub validate: Option<String>,
    pub target_type: Option<u8>,
    pub target_user_id: Option<u64>,
    pub target_application_id: Option<u64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct InviteMetadata {
    pub uses: Option<u64>,
    pub max_uses: Option<u64>,
    pub max_age: Option<u64>,
    pub temporary: Option<bool>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    pub created_at: DateTime<Utc>,
}
