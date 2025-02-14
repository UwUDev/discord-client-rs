use crate::deserializer::deserialize_string_to_u64;
use crate::structs::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Team {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
    pub icon: Option<String>,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub owner_user_id: u64,
    pub members: Option<Vec<TeamMember>>,
    pub payout_account_status: Option<u64>,
    pub stripe_connect_account_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TeamMember {
    pub user: User,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub team_id: u64,
    pub membership_state: u64,
    pub role: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Company {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub id: u64,
    pub name: String,
}
