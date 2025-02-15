use discord_client_structs::structs::misc::UserSettingsProto;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ContentInventoryInboxStaleEvent {
    pub refresh_after_ms: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UserSettingsProtoUpdateEvent {
    pub settings: UserSettingsProto,
    pub partial: bool,
}
