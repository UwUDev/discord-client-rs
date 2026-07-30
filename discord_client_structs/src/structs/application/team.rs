use crate::structs::user::User;
use discord_client_macros::discord_struct;

#[discord_struct]
pub struct Team {
    #[snowflake]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
    #[snowflake]
    pub owner_user_id: u64,
    pub members: Option<Vec<TeamMember>>,
    pub payout_account_status: Option<u64>,
    pub stripe_connect_account_id: Option<String>,
}

#[discord_struct]
pub struct TeamMember {
    pub user: User,
    #[snowflake]
    pub team_id: u64,
    pub membership_state: u64,
    pub role: String,
}

#[discord_struct]
pub struct Company {
    #[snowflake]
    pub id: u64,
    pub name: String,
}
