use discord_client_macros::discord_struct;

#[discord_struct]
pub struct GameActivity {
    pub activity_level: u32,
    pub activity_score: u32,
}
