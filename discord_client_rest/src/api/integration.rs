use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, HomePageReferer, Referer};
use serde_json::Value;
use std::collections::HashMap;

pub struct IntegrationRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> IntegrationRest<'a> {
    fn home_props(&self) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(HomePageReferer {}.into())
            .build()?)
    }

    pub async fn get_guild_integrations(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/integrations", guild_id);

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(GuildReferer { guild_id }.into())
            .build()?;

        self.client.get::<Value>(&path, None, Some(props)).await
    }

    pub async fn get_integration_application_ids(&self) -> BoxedResult<Value> {
        let path = "users/@me/guilds/integration-application-ids";

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_trending_gifs(&self) -> BoxedResult<Value> {
        let path = "gifs/trending";

        let mut query = HashMap::new();
        query.insert("media_format".to_string(), "mp4".to_string());
        query.insert("provider".to_string(), "tenor".to_string());

        self.client
            .get(&path, Some(query), Some(self.home_props()?))
            .await
    }

    pub async fn search_gifs(&self, q: String) -> BoxedResult<Value> {
        let path = "gifs/search";

        let mut query = HashMap::new();
        query.insert("q".to_string(), q);
        query.insert("media_format".to_string(), "mp4".to_string());
        query.insert("provider".to_string(), "tenor".to_string());

        self.client
            .get(&path, Some(query), Some(self.home_props()?))
            .await
    }
}
