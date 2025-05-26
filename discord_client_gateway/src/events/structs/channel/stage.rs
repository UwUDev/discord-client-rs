use discord_client_structs::structs::guild::stage::StageInstance;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StageInstanceCreateEvent {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StageInstanceUpdateEvent {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}

#[derive(Debug, Deserialize, Clone)]
pub struct StageInstanceDeleteEvent {
    #[serde(flatten)]
    pub stage_instance: StageInstance,
}
