use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct EntitlementRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> EntitlementRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_entitlements(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/entitlements").await
    }

    pub async fn get_gift_entitlements(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/entitlements/gifts").await
    }

    pub async fn get_guild_entitlements(&self, guild_id: u64) -> BoxedResult<Value> {
        self.client
            .get::<Value>(
                &format!("guilds/{}/entitlements", guild_id),
                None,
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn get_application_entitlements(&self, application_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!(
            "users/@me/applications/{}/entitlements",
            application_id
        ))
        .await
    }

    pub async fn get_gift_code(&self, code: String) -> BoxedResult<Value> {
        self.home_get(&format!("entitlements/gift-codes/{}", code))
            .await
    }
}
