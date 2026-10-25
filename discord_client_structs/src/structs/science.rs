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
    pub distributor: Option<String>,
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
    pub game_distributor: Option<String>,
    pub game_distributor_game_id: Option<String>,
    pub game_detection_enabled: bool,
    pub initial_heartbeat: bool,
    pub final_heartbeat: bool,
    pub game_session_id: String,
    /// Milliseconds elapsed since the previous heartbeat, 0 on the initial heartbeat.
    pub duration_tracked_ms: i64,
    pub discord_window_state: Option<Value>,
    pub rtc_connection_id: Option<String>,
    pub media_session_id: Option<String>,
    pub launch_signature: String,
    pub client_app_state: String,
    pub client_send_timestamp: i64,
}
