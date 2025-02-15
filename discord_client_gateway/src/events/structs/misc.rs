use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ContentInventoryInboxStaleEvent {
    pub refresh_after_ms: u64,
}
