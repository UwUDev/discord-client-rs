use discord_client_structs::deserializer::deserialize_string_to_u64;
use discord_client_structs::structs::message::soundboard::SoundboardSound;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct GuildSoundboardSoundCreateEvent {
    #[serde(flatten)]
    pub sound: SoundboardSound,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildSoundboardSoundUpdateEvent {
    #[serde(flatten)]
    pub sound: SoundboardSound,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GuildSoundboardSoundDeleteEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub sound_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SoundboardSoundsEvent {
    #[serde(deserialize_with = "deserialize_string_to_u64")]
    pub guild_id: u64,
    pub soundboard_sounds: Vec<SoundboardSound>,
}
