use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct CollectiblesRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> CollectiblesRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_categories(&self) -> BoxedResult<Value> {
        self.home_get("collectibles-categories").await
    }

    pub async fn get_categories_v2(&self) -> BoxedResult<Value> {
        self.home_get("collectibles-categories/v2").await
    }

    pub async fn get_shop(&self) -> BoxedResult<Value> {
        self.home_get("collectibles-shop").await
    }

    pub async fn get_product(&self, sku_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("collectibles-products/{}", sku_id))
            .await
    }

    pub async fn get_purchases(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/collectibles-purchases").await
    }
}
