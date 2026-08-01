use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct PresenceRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> PresenceRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_activity_statistics(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/activities/statistics/applications")
            .await
    }
}
