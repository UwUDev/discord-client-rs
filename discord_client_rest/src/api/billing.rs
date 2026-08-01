use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct BillingRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> BillingRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_country_code(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/country-code").await
    }

    pub async fn get_location_info(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/location-info").await
    }

    pub async fn get_payment_sources(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/payment-sources").await
    }

    pub async fn get_payment_source(&self, id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("users/@me/billing/payment-sources/{}", id))
            .await
    }

    pub async fn get_payments(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/payments").await
    }

    pub async fn get_payment(&self, id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("users/@me/billing/payments/{}", id))
            .await
    }

    pub async fn get_invoice_breakdown(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/invoice/breakdown").await
    }

    pub async fn get_localized_pricing_promo(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/localized-pricing-promo")
            .await
    }

    pub async fn get_checkout_recovery(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/checkout-recovery").await
    }

    pub async fn get_nitro_affinity(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/nitro-affinity").await
    }

    pub async fn get_trial_offer(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/user-trial-offer").await
    }

    pub async fn get_subscriptions(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/billing/subscriptions").await
    }

    pub async fn get_subscription(&self, id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("users/@me/billing/subscriptions/{}", id))
            .await
    }

    pub async fn get_premium_guild_subscriptions(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/guilds/premium/subscriptions")
            .await
    }

    pub async fn get_premium_guild_subscription_slots(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/guilds/premium/subscription-slots")
            .await
    }
}
