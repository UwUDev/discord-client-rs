use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::guild::clan::GuildJoinRequest;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GuildJoinRequestCreateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub request: GuildJoinRequest,
    #[serde(deserialize_with = "deserialize_guild_join_request_status")]
    pub status: JoinRequestStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildJoinRequestUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub request: GuildJoinRequest,
    #[serde(deserialize_with = "deserialize_guild_join_request_status")]
    pub status: JoinRequestStatus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GuildJoinRequestDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub user_id: u64,
}

// Enum
#[derive(Debug, Clone)]
pub enum JoinRequestStatus {
    STARTED,
    SUBMITTED,
    REJECTED,
    APPROVED,
}

impl JoinRequestStatus {
    pub fn from_str(status: &str) -> Option<Self> {
        match status {
            "STARTED" => Some(JoinRequestStatus::STARTED),
            "SUBMITTED" => Some(JoinRequestStatus::SUBMITTED),
            "REJECTED" => Some(JoinRequestStatus::REJECTED),
            "APPROVED" => Some(JoinRequestStatus::APPROVED),
            _ => None,
        }
    }
}

impl std::fmt::Display for JoinRequestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinRequestStatus::STARTED => write!(f, "STARTED"),
            JoinRequestStatus::SUBMITTED => write!(f, "SUBMITTED"),
            JoinRequestStatus::REJECTED => write!(f, "REJECTED"),
            JoinRequestStatus::APPROVED => write!(f, "APPROVED"),
        }
    }
}

fn deserialize_guild_join_request_status<'de, D>(
    deserializer: D,
) -> Result<JoinRequestStatus, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    JoinRequestStatus::from_str(&s).ok_or(serde::de::Error::custom(format!(
        "Invalid GuildJoinRequestStatus: {}",
        s
    )))
}
