use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, HomePageReferer, Referer};
use discord_client_structs::structs::channel::voice::VoiceState;
use serde_json::Value;

pub struct VoiceRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> VoiceRest<'a> {
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

    pub async fn get_regions(&self) -> BoxedResult<Value> {
        let path = "voice/regions";

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_guild_regions(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/regions", guild_id);

        self.client
            .get::<Value>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn get_current_state(&self, guild_id: u64) -> BoxedResult<VoiceState> {
        let path = format!("guilds/{}/voice-states/@me", guild_id);

        self.client
            .get::<VoiceState>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn get_state(&self, guild_id: u64, user_id: u64) -> BoxedResult<VoiceState> {
        let path = format!("guilds/{}/voice-states/{}", guild_id, user_id);

        self.client
            .get::<VoiceState>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn modify_current_state(&self, guild_id: u64, state: Value) -> BoxedResult<()> {
        let path = format!("guilds/{}/voice-states/@me", guild_id);

        self.client
            .patch::<(), Value>(&path, Some(state), Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn modify_state(&self, guild_id: u64, user_id: u64, state: Value) -> BoxedResult<()> {
        let path = format!("guilds/{}/voice-states/{}", guild_id, user_id);

        self.client
            .patch::<(), Value>(&path, Some(state), Some(self.guild_props(guild_id)?))
            .await
    }
}
