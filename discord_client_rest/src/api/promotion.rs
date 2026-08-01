use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct PromotionRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> PromotionRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_outbound_promotions(&self) -> BoxedResult<Value> {
        self.home_get("outbound-promotions").await
    }

    pub async fn get_promotions(&self) -> BoxedResult<Value> {
        self.home_get("promotions").await
    }

    pub async fn get_bogo_promotions(&self) -> BoxedResult<Value> {
        self.home_get("bogo-promotions").await
    }

    pub async fn get_outbound_promotion_codes(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/outbound-promotions/codes").await
    }
}
