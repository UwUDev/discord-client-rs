use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct SafetyHubRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> SafetyHubRest<'a> {
    pub async fn get(&self) -> BoxedResult<Value> {
        self.client
            .get::<Value>("safety-hub/@me", None, Some(RequestProperties::home()))
            .await
    }
}
