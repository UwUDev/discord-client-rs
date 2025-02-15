use discord_client_structs::structs::guild::GatewayGuild;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ReadyEvent {
    pub session_id: String,
    pub analytics_token: String,
    pub auth_session_id_hash: String,
    pub guilds: Vec<GatewayGuild>,
}
