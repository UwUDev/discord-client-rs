use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct StoreRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> StoreRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_sku(&self, sku_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/skus/{}", sku_id)).await
    }

    pub async fn get_sku_listings(&self, sku_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/skus/{}/listings", sku_id))
            .await
    }

    pub async fn get_sku_plans(&self, sku_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/skus/{}/plans", sku_id)).await
    }

    pub async fn get_published_listings_sku(&self, sku_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/published-listings/skus/{}", sku_id))
            .await
    }

    pub async fn get_application_published_listings(
        &self,
        application_id: u64,
    ) -> BoxedResult<Value> {
        self.home_get(&format!(
            "store/published-listings/applications/{}",
            application_id
        ))
        .await
    }

    pub async fn get_price_tiers(&self) -> BoxedResult<Value> {
        self.home_get("store/price-tiers").await
    }

    pub async fn get_price_tier(&self, price_tier: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/price-tiers/{}", price_tier))
            .await
    }

    pub async fn get_eula(&self, eula_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("store/eulas/{}", eula_id)).await
    }

    pub async fn get_virtual_currency_balance(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/virtual-currency/balance").await
    }
}
