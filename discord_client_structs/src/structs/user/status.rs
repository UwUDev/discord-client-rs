use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    Online,
    DoNotDisturb,
    AFK,
    Invisible,
    Offline,
    Unknown,
}

impl StatusType {
    pub fn as_str(&self) -> &str {
        match self {
            StatusType::Online => "online",
            StatusType::DoNotDisturb => "dnd",
            StatusType::AFK => "idle",
            StatusType::Invisible => "invisible",
            StatusType::Offline => "offline",
            StatusType::Unknown => "unknown",
        }
    }

    pub fn from_str(s: &str) -> StatusType {
        match s {
            "online" => StatusType::Online,
            "dnd" => StatusType::DoNotDisturb,
            "idle" => StatusType::AFK,
            "invisible" => StatusType::Invisible,
            "offline" => StatusType::Offline,
            "unknown" => StatusType::Unknown,
            _ => StatusType::Unknown,
        }
    }
}

impl Serialize for StatusType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for StatusType {
    fn deserialize<D>(deserializer: D) -> Result<StatusType, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(StatusType::from_str(&s))
    }
}
