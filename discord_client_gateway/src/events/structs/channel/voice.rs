use discord_client_macros::discord_struct;
use discord_client_structs::deserializer::deserialize_option_string_to_u64;
use discord_client_structs::structs::channel::voice::VoiceState;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceStateUpdateEvent {
    #[serde(flatten)]
    pub voice_state: VoiceState,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceStateUpdateBatchEvent {
    #[serde(default)]
    pub voice_states: Vec<VoiceState>,
}

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct VoiceChannelStatusUpdateEvent {
    pub status: Option<String>,
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
}

#[discord_struct(no_builder, no_default, no_serialize)]
pub struct VoiceChannelStartTimeUpdateEvent {
    #[snowflake]
    pub id: u64,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    pub voice_start_time: Option<i64>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct VoiceServerUpdateEvent {
    pub token: String,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub guild_id: Option<u64>,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    pub channel_id: Option<u64>,
    pub endpoint: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::{
        VoiceChannelStartTimeUpdateEvent, VoiceServerUpdateEvent, VoiceStateUpdateBatchEvent,
    };

    #[test]
    fn voice_state_update_batch() {
        let raw = r#"{"voice_states":[{"user_id":"1","suppress":false,"session_id":"abc","self_video":false,"self_mute":false,"self_deaf":false,"mute":false,"deaf":false,"channel_id":"2"}]}"#;
        let e: VoiceStateUpdateBatchEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.voice_states.len(), 1);
        assert_eq!(e.voice_states[0].user_id, 1);
        assert_eq!(e.voice_states[0].channel_id, Some(2));
    }

    #[test]
    fn start_time_update_ended() {
        let raw = r#"{"voice_start_time":null,"id":"1274353848894492682","guild_id":"1213802748240199690"}"#;
        let e: VoiceChannelStartTimeUpdateEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.id, 1274353848894492682);
        assert_eq!(e.guild_id, Some(1213802748240199690));
        assert_eq!(e.voice_start_time, None);
    }

    #[test]
    fn start_time_update_started() {
        let raw = r#"{"voice_start_time":1700000000,"id":"1","guild_id":"2"}"#;
        let e: VoiceChannelStartTimeUpdateEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.voice_start_time, Some(1700000000));
    }

    #[test]
    fn voice_server_update_null_guild() {
        let raw = r#"{"token":"66d29164ee8cd919","guild_id":null,"endpoint":"smart.loyal.discord.media:1337"}"#;
        let e: VoiceServerUpdateEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(e.token, "66d29164ee8cd919");
        assert_eq!(e.guild_id, None);
        assert_eq!(
            e.endpoint.as_deref(),
            Some("smart.loyal.discord.media:1337")
        );
    }
}
