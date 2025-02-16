use crate::structs::user::deserialize_string_to_u64;
use serde::Deserialize;

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
    r#type: String,
    pub account: Account,
    pub guild: IntegrationGuild,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub name: String,
}

// Integration Guild Structure
//
// Field	Type	Description
// id	snowflake	The ID of the guild
// name	string	The name of the guild (2-100 characters)
// icon	?string	The guild's icon hash

#[derive(Debug, Deserialize, Clone)]
pub struct IntegrationGuild {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
}

impl ConnectionIntegration {
    pub fn get_type(&self) -> ConnectionType {
        match self.r#type.as_str() {
            "amazon-music" => ConnectionType::AmazonMusic,
            "battlenet" => ConnectionType::BattleNet,
            "bluesky" => ConnectionType::BlueSky,
            "bungie" => ConnectionType::Bungie,
            "contacts 2" => ConnectionType::ContactSync,
            "crunchyroll" => ConnectionType::Crunchyroll,
            "domain" => ConnectionType::Domain,
            "ebay" => ConnectionType::Ebay,
            "epicgames" => ConnectionType::EpicGames,
            "facebook" => ConnectionType::Facebook,
            "github" => ConnectionType::GitHub,
            "instagram 1" => ConnectionType::Instagram,
            "leagueoflegends" => ConnectionType::LeagueOfLegends,
            "mastodon" => ConnectionType::Mastodon,
            "paypal" => ConnectionType::PayPal,
            "playstation" => ConnectionType::PlayStation,
            "playstation-stg" => ConnectionType::PlayStationStaging,
            "reddit" => ConnectionType::Reddit,
            "roblox" => ConnectionType::Roblox,
            "riotgames" => ConnectionType::RiotGames,
            "samsung 1" => ConnectionType::Samsung,
            "soundcloud" => ConnectionType::SoundCloud,
            "spotify" => ConnectionType::Spotify,
            "skype 1" => ConnectionType::Skype,
            "steam" => ConnectionType::Steam,
            "tiktok" => ConnectionType::TikTok,
            "twitch" => ConnectionType::Twitch,
            "twitter" => ConnectionType::Twitter,
            "xbox" => ConnectionType::Xbox,
            "youtube" => ConnectionType::YouTube,
            _ => ConnectionType::Unknown(self.r#type.clone()),
        }
    }
}

pub enum ConnectionType {
    AmazonMusic,
    BattleNet,
    BlueSky,
    Bungie,
    ContactSync,
    Crunchyroll,
    Domain,
    Ebay,
    EpicGames,
    Facebook,
    GitHub,
    Instagram,
    LeagueOfLegends,
    Mastodon,
    PayPal,
    PlayStation,
    PlayStationStaging,
    Reddit,
    Roblox,
    RiotGames,
    Samsung,
    SoundCloud,
    Spotify,
    Skype,
    Steam,
    TikTok,
    Twitch,
    Twitter,
    Xbox,
    YouTube,
    Unknown(String),
}