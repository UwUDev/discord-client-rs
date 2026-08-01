use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;
use std::collections::HashMap;

pub struct DiscoveryRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> DiscoveryRest<'a> {
    pub async fn get_categories(&self) -> BoxedResult<Value> {
        let path = "discovery/categories";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn validate_search_term(&self, term: String) -> BoxedResult<Value> {
        let path = "discovery/valid-term";

        let mut query = HashMap::new();
        query.insert("term".to_string(), term);

        self.client
            .get(&path, Some(query), Some(RequestProperties::home()))
            .await
    }

    pub async fn get_guild_requirements(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/discovery-requirements", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_guild_metadata(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/discovery-metadata", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }
}
