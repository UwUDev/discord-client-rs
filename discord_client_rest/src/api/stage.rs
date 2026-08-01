use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::guild::stage::StageInstance;
use serde_json::Value;

pub struct StageRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> StageRest<'a> {
    pub async fn create(&self, stage: Value) -> BoxedResult<StageInstance> {
        let path = "stage-instances";

        self.client
            .post::<StageInstance, Value>(&path, Some(stage), Some(RequestProperties::home()))
            .await
    }

    pub async fn get(&self, channel_id: u64) -> BoxedResult<StageInstance> {
        let path = format!("stage-instances/{}", channel_id);

        self.client
            .get::<StageInstance>(&path, None, Some(RequestProperties::home()))
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
            .patch::<StageInstance, Value>(
                &path,
                Some(stage),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn delete(&self, channel_id: u64, guild_id: u64) -> BoxedResult<()> {
        let path = format!("stage-instances/{}", channel_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }
}
