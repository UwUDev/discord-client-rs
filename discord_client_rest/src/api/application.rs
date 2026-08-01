use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct ApplicationRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> ApplicationRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_applications(&self) -> BoxedResult<Value> {
        self.home_get("applications").await
    }

    pub async fn get_application(&self, application_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("applications/{}", application_id))
            .await
    }

    pub async fn get_public_application(&self, application_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("applications/{}/public", application_id))
            .await
    }

    pub async fn get_public_applications(&self, application_ids: Vec<u64>) -> BoxedResult<Value> {
        let mut query = std::collections::HashMap::new();
        query.insert(
            "application_ids".to_string(),
            application_ids
                .iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(","),
        );

        self.client
            .get(
                "applications/public",
                Some(query),
                Some(RequestProperties::home()),
            )
            .await
    }

    pub async fn get_application_rpc(&self, application_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("applications/{}/rpc", application_id))
            .await
    }

    pub async fn get_application_assets(&self, application_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("applications/{}/assets", application_id))
            .await
    }

    pub async fn get_guild_applications(&self, guild_id: u64) -> BoxedResult<Value> {
        self.client
            .get::<Value>(
                &format!("guilds/{}/applications", guild_id),
                None,
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn get_role_connections(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/applications/role-connections")
            .await
    }

    pub async fn get_detectable_applications(&self) -> BoxedResult<Value> {
        self.home_get("applications/detectable").await
    }
}
