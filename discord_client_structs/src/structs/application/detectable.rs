use discord_client_macros::discord_struct;
use serde::{Deserialize, Serialize};

#[discord_struct]
pub struct DetectableApplication {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub icon_hash: Option<String>,
    pub cover_image_hash: Option<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub executables: Vec<ApplicationExecutable>,
    #[serde(default)]
    pub themes: Vec<String>,
    #[serde(default)]
    pub hook: bool,
    #[serde(default)]
    pub overlay: bool,
    pub overlay_methods: Option<u64>,
    #[serde(default)]
    pub overlay_warn: bool,
    #[serde(default)]
    pub overlay_compatibility_hook: bool,
    pub linked_applications: Option<Vec<LinkedApplication>>,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct ApplicationExecutable {
    pub os: String,
    pub name: String,
    #[serde(default)]
    pub is_launcher: bool,
}

#[discord_struct]
pub struct LinkedApplication {
    #[snowflake]
    pub id: u64,
    pub r#type: u8,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct DetectableExclusions {
    #[serde(default)]
    pub executables: Vec<String>,
    #[serde(default)]
    pub patterns: Vec<String>,
}
