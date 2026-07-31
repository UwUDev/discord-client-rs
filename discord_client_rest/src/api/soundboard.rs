use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildChannelReferer, GuildReferer, HomePageReferer, Referer};
use discord_client_structs::structs::message::soundboard::SoundboardSound;
use serde_json::Value;

pub struct SoundboardRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> SoundboardRest<'a> {
    fn home_props(&self) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(HomePageReferer {}.into())
            .build()?)
    }

    fn guild_props(&self, guild_id: u64) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(GuildReferer { guild_id }.into())
            .build()?)
    }

    pub async fn get_default_sounds(&self) -> BoxedResult<Vec<SoundboardSound>> {
        let path = "soundboard-default-sounds";

        self.client
            .get::<Vec<SoundboardSound>>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_guild_sounds(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/soundboard-sounds", guild_id);

        self.client
            .get::<Value>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn get_guild_sound(
        &self,
        guild_id: u64,
        sound_id: u64,
    ) -> BoxedResult<SoundboardSound> {
        let path = format!("guilds/{}/soundboard-sounds/{}", guild_id, sound_id);

        self.client
            .get::<SoundboardSound>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn get_sound_guild(&self, sound_id: u64, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("soundboard-sounds/{}/guild/{}", sound_id, guild_id);

        self.client
            .get::<Value>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn modify_guild_sound(
        &self,
        guild_id: u64,
        sound_id: u64,
        sound: Value,
    ) -> BoxedResult<SoundboardSound> {
        let path = format!("guilds/{}/soundboard-sounds/{}", guild_id, sound_id);

        self.client
            .patch::<SoundboardSound, Value>(&path, Some(sound), Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn delete_guild_sound(&self, guild_id: u64, sound_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/soundboard-sounds/{}", guild_id, sound_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn send(&self, channel_id: u64, guild_id: u64, sound: Value) -> BoxedResult<()> {
        let path = format!("channels/{}/send-soundboard-sound", channel_id);

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(
                GuildChannelReferer {
                    guild_id,
                    channel_id,
                }
                .into(),
            )
            .build()?;

        self.client
            .post::<(), Value>(&path, Some(sound), Some(props))
            .await
    }
}
