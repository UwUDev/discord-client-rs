use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct QuestsRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> QuestsRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_quests(&self) -> BoxedResult<Value> {
        self.home_get("quests/@me").await
    }

    pub async fn get_claimed_quests(&self) -> BoxedResult<Value> {
        self.home_get("quests/@me/claimed").await
    }

    pub async fn get_quest(&self, quest_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("quests/{}", quest_id)).await
    }

    pub async fn get_quest_preview(&self, quest_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("quests/{}/preview", quest_id)).await
    }
}
