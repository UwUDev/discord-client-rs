use crate::structs::user::User;
use crate::structs::user::activity::Activity;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Presence {
    #[serde(flatten)]
    pub user: Option<User>,
    #[serde(default)]
    #[snowflake]
    pub guild_id: Option<u64>,
    pub status: String,
    pub activities: Vec<Activity>,
    pub client_status: ClientStatus,
}

#[discord_struct]
pub struct ClientStatus {
    pub desktop: Option<String>,
    pub mobile: Option<String>,
    pub web: Option<String>,
    pub embedded: Option<String>,
}

#[discord_struct]
pub struct MergedPresences {
    pub friends: Vec<Presence>,
    pub guilds: Vec<Vec<Presence>>,
}
