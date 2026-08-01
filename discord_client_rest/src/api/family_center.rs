use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct FamilyCenterRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> FamilyCenterRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_overview(&self) -> BoxedResult<Value> {
        self.home_get("family-center/@me").await
    }

    pub async fn get_linked_users(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/linked-users").await
    }
}
