use discord_client_macros::discord_struct;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "properties", rename_all = "snake_case")]
pub enum ScienceEvent {
    LaunchGame(LaunchGame),
    RunningGameHeartbeat(RunningGameHeartbeat),
}

#[discord_struct]
pub struct LaunchGame {
    pub client_track_timestamp: i64,
    pub client_heartbeat_session_id: String,
    pub event_sequence_number: u64,
    pub game: String,
    #[snowflake]
    pub game_id: u64,
    pub verified: bool,
    pub elevated: bool,
    pub is_launcher: bool,
    pub game_platform: String,
    pub detection_method: String,
    pub is_overlay_enabled: bool,
    pub is_overlay_game_enabled: bool,
    pub is_overlay_game_source: String,
    pub fullscreen_type: String,
    pub hardware_display_count: u32,
    pub overlay_method: String,
    pub activity_status_enabled: bool,
    #[snowflake]
    pub activity_status_shared_guilds: Vec<u64>,
    pub current_user_status: String,
    pub game_detection_enabled: bool,
    pub executable_path: String,
    #[snowflake]
    pub voice_channel_id: Option<u64>,
    pub voice_channel_type: Option<u32>,
    pub voice_channel_bitrate: Option<u32>,
    #[snowflake]
    pub voice_channel_guild_id: Option<u64>,
    pub hidden_by_distributor: bool,
    pub game_metadata: Option<Value>,
    pub executable_fingerprint: Option<String>,
    pub client_performance_cpu: Option<f64>,
    pub client_performance_memory: Option<f64>,
    pub cpu_core_count: Option<u32>,
    pub accessibility_features: u32,
    pub rendered_locale: String,
    pub launch_signature: String,
    pub client_rtc_state: Option<Value>,
    pub client_app_state: String,
    pub client_send_timestamp: i64,
}

#[discord_struct]
pub struct RunningGameHeartbeat {
    pub client_track_timestamp: i64,
    pub client_heartbeat_session_id: String,
    pub event_sequence_number: u64,
    #[snowflake]
    pub game_id: u64,
    pub game_name: String,
    pub game_metadata: Option<Value>,
    pub game_executable: String,
    pub game_detection_enabled: bool,
    pub initial_heartbeat: bool,
    pub final_heartbeat: bool,
    pub game_session_id: String,
    pub duration_tracked_ms: i64,
    pub rtc_connection_id: Option<String>,
    pub media_session_id: Option<String>,
    pub launch_signature: String,
    pub client_app_state: String,
    pub client_send_timestamp: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_game_serializes_with_type_and_properties() {
        let value = serde_json::to_value(ScienceEvent::LaunchGame(LaunchGame::default())).unwrap();
        assert_eq!(value["type"], "launch_game");
        assert!(value["properties"].is_object());
    }

    #[test]
    fn heartbeat_serializes_with_type_and_properties() {
        let value =
            serde_json::to_value(ScienceEvent::RunningGameHeartbeat(RunningGameHeartbeat::default()))
                .unwrap();
        assert_eq!(value["type"], "running_game_heartbeat");
        assert!(value["properties"].is_object());
    }

    #[test]
    fn snowflake_ids_serialize_as_strings() {
        let mut launch = LaunchGame::default();
        launch.game_id = 1124354429882220574;
        let value = serde_json::to_value(ScienceEvent::LaunchGame(launch)).unwrap();
        assert!(value["properties"]["game_id"].is_string());
    }
}
