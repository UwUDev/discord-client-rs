use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct PremiumReferralRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> PremiumReferralRest<'a> {
    async fn home_get(&self, path: &str) -> BoxedResult<Value> {
        self.client
            .get::<Value>(path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_referral(&self, referral_id: u64) -> BoxedResult<Value> {
        self.home_get(&format!("referrals/{}", referral_id)).await
    }

    pub async fn get_incentive_eligibility(&self) -> BoxedResult<Value> {
        self.home_get("users/@me/referrals/incentive-eligibility")
            .await
    }
}
