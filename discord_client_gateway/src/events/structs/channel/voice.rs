use discord_client_macros::CreatedAt;
use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::channel::voice::VoiceState;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceStateUpdateEvent {
    #[serde(flatten)]
    pub voice_state: VoiceState,
}

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct VoiceChannelStatusUpdateEvent {
    pub status: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
}

#[derive(Debug, Deserialize, Clone, CreatedAt)]
pub struct VoiceChannelStartTimeUpdateEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    /// Unix timestamp (seconds) the current voice session started, or `None` if ended.
    pub voice_start_time: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::VoiceChannelStartTimeUpdateEvent;

    // Real captured payload (session ended -> voice_start_time null).
    #[test]
    fn parses_ended_session() {
        let raw = r#"{"voice_start_time":null,"id":"1274353848894492682","guild_id":"1213802748240199690"}"#;
        let e: VoiceChannelStartTimeUpdateEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.id, 1274353848894492682);
        assert_eq!(e.guild_id, Some(1213802748240199690));
        assert_eq!(e.voice_start_time, None);
    }

    #[test]
    fn parses_started_session() {
        let raw = r#"{"voice_start_time":1700000000,"id":"1","guild_id":"2"}"#;
        let e: VoiceChannelStartTimeUpdateEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.voice_start_time, Some(1700000000));
    }
}
