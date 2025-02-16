use discord_client_structs::structs::user::connection::Connection;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct UserConnectionsUpdateEvent {
    #[serde(flatten)]
    pub connection: Option<Connection>,
    pub user_id: u64,
}
