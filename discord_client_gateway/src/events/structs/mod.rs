use serde_json::Value;

pub mod application;
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
pub mod stream;
pub mod user;

#[derive(Debug, Clone)]
pub struct UnknownEvent {
    pub r#type: String,
    pub data: Value,
    pub op: u8,
}

#[derive(Debug, Clone)]
pub struct ParseErrorEvent {
    pub event_type: String,
    pub op: u8,
    pub error: String,
    pub path: String,
    pub raw: Value,
}

impl ParseErrorEvent {
    pub fn dump_to<P: AsRef<std::path::Path>>(
        &self,
        dir: P,
    ) -> std::io::Result<std::path::PathBuf> {
        let dir = dir.as_ref();
        std::fs::create_dir_all(dir)?;
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let path = dir.join(format!("{}-{}.json", self.event_type, nanos));
        let json = serde_json::to_string_pretty(&self.raw).unwrap_or_else(|_| self.raw.to_string());
        std::fs::write(&path, json)?;
        Ok(path)
    }
}
