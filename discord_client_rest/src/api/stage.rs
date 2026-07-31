use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, HomePageReferer, Referer};
use discord_client_structs::structs::guild::stage::StageInstance;
use serde_json::Value;

pub struct StageRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> StageRest<'a> {
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

    pub async fn create(&self, stage: Value) -> BoxedResult<StageInstance> {
        let path = "stage-instances";

        self.client
            .post::<StageInstance, Value>(&path, Some(stage), Some(self.home_props()?))
            .await
    }

    pub async fn get(&self, channel_id: u64) -> BoxedResult<StageInstance> {
        let path = format!("stage-instances/{}", channel_id);

        self.client
            .get::<StageInstance>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn modify(
        &self,
        channel_id: u64,
        guild_id: u64,
        stage: Value,
    ) -> BoxedResult<StageInstance> {
        let path = format!("stage-instances/{}", channel_id);

        self.client
            .patch::<StageInstance, Value>(&path, Some(stage), Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn delete(&self, channel_id: u64, guild_id: u64) -> BoxedResult<()> {
        let path = format!("stage-instances/{}", channel_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.guild_props(guild_id)?))
            .await
    }
}
