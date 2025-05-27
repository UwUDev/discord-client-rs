use crate::deserializer::*;
use crate::serializer::*;
use crate::structs::guild::integration::*;
use discord_client_macros::{CreatedAt, EnumFromString};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Connection {
    pub id: String,
    pub r#type: String,
    pub name: String,
    pub verified: bool,
    pub metadata: Option<serde_json::Value>,
    pub metadata_visibility: Option<u8>,
    pub revoked: bool,
    pub integrations: Option<Vec<ConnectionIntegration>>,
    pub friend_sync: bool,
    pub show_activity: bool,
    pub two_way_link: bool,
    pub visibility: u8,
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ConnectionIntegration {
    pub id: String,
    pub r#type: ConnectionType,
    pub account: Account,
    pub guild: IntegrationGuild,
}

#[derive(Debug, Deserialize, Serialize, Clone, CreatedAt)]
pub struct PartialIntegration {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub r#type: IntegrationType,
    pub account: Account,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_vec_u64")]
    pub application_id: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, CreatedAt)]
pub struct IntegrationGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    #[serde(serialize_with = "serialize_u64_as_string")]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromString)]
pub enum ConnectionType {
    #[str_value("amazon-music")]
    AmazonMusic,
    BattleNet,
    BlueSky,
    Bungie,
    #[str_value("contacts 2")]
    ContactSync,
    Crunchyroll,
    Domain,
    Ebay,
    EpicGames,
    Facebook,
    GitHub,
    #[str_value("instagram 1")]
    Instagram,
    LeagueOfLegends,
    Mastodon,
    PayPal,
    PlayStation,
    #[str_value("playstation-stg")]
    PlayStationStaging,
    Reddit,
    Roblox,
    RiotGames,
    #[str_value("samsung 1")]
    Samsung,
    SoundCloud,
    Spotify,
    #[str_value("skype 1")]
    Skype,
    Steam,
    TikTok,
    Twitch,
    Twitter,
    Xbox,
    YouTube,
    Unknown,
}
