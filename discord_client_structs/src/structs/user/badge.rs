use crate::deserializer::*;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Default, Deserialize, Clone)]
pub struct UserBadges {
    pub badges: Vec<UserBadge>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserBadge {
    pub badge_id: u64,
    pub name: String,
    pub rarity: u8,
    pub simple_icon_url: String,
    pub simple_icon_raster_url: String,
    pub is_earnable: bool,
    pub owned: bool,
    pub hidden: bool,
    pub description: Option<String>,
    pub leveling_instructions: Option<String>,
    pub info_label: Option<String>,
    pub complex_icon_static_url: Option<String>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    pub obtained_at: Option<DateTime<Utc>>,
    pub current_tier: Option<String>,
    pub next_tier: Option<String>,
    pub tier_obtained_at: Option<HashMap<String, String>>,
    pub tiers: Option<Vec<BadgeTier>>,
    pub progress: Option<Vec<BadgeProgress>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BadgeTier {
    pub key: String,
    pub name: Option<String>,
    pub rarity: u8,
    pub simple_icon_url: String,
    pub simple_icon_raster_url: String,
    pub complex_icon_static_url: Option<String>,
    pub complex_icon_animated_url: Option<String>,
    pub requirements: Vec<BadgeRequirement>,
    pub owned: bool,
    pub milestone_text: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BadgeRequirement {
    pub progress_type: u8,
    pub threshold: Option<u64>,
    pub unit: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BadgeProgress {
    pub progress_type: u8,
    pub current: Option<u64>,
    pub floor: Option<u64>,
    pub threshold: Option<u64>,
    pub unit: Option<String>,
    pub window: Option<serde_json::Value>,
    pub progress_helper_text: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_badges() {
        let raw = r#"{"badges":[{"badge_id":22,"name":"A","rarity":5,"simple_icon_url":"u","simple_icon_raster_url":"r","is_earnable":false,"owned":true,"hidden":false,"obtained_at":"2020-01-01T00:00:00.000000+00:00","tiers":[{"key":"k","rarity":1,"simple_icon_url":"u","simple_icon_raster_url":"r","requirements":[{"progress_type":1,"threshold":2}],"owned":false}],"progress":[{"progress_type":1,"current":0,"floor":0,"threshold":2,"unit":null,"window":null,"progress_helper_text":"x"}]}]}"#;
        let parsed: UserBadges = serde_json::from_str(raw).unwrap();
        assert_eq!(parsed.badges[0].badge_id, 22);
        assert!(parsed.badges[0].obtained_at.is_some());
    }
}
