use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;
use std::collections::HashMap;

pub struct IntegrationRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> IntegrationRest<'a> {
    pub async fn get_guild_integrations(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/integrations", guild_id);

        let props = RequestProperties::guild(guild_id);

        self.client.get::<Value>(&path, None, Some(props)).await
    }

    pub async fn get_integration_application_ids(&self) -> BoxedResult<Value> {
        let path = "users/@me/guilds/integration-application-ids";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_trending_gifs(&self) -> BoxedResult<Value> {
        let path = "gifs/trending";

        let mut query = HashMap::new();
        query.insert("media_format".to_string(), "mp4".to_string());
        query.insert("provider".to_string(), "tenor".to_string());

        self.client
            .get(&path, Some(query), Some(RequestProperties::home()))
            .await
    }

    pub async fn search_gifs(&self, q: String) -> BoxedResult<Value> {
        let path = "gifs/search";

        let mut query = HashMap::new();
        query.insert("q".to_string(), q);
        query.insert("media_format".to_string(), "mp4".to_string());
        query.insert("provider".to_string(), "tenor".to_string());

        self.client
            .get(&path, Some(query), Some(RequestProperties::home()))
            .await
    }
}
