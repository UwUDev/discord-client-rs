use serde_json::Value;

pub mod call;
pub mod channel;
pub mod gateway;
pub mod guild;
pub mod message;
pub mod misc;
pub mod notifications;
pub mod presence;
pub mod ready;
pub mod requested;
pub mod user;

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub r#type: String,
    pub data: Value,
    pub op: u8,
}
