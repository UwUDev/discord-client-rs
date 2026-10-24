use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::science::ScienceEvent;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct ScienceRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> ScienceRest<'a> {
    pub async fn analytics_token(&self) -> BoxedResult<String> {
        if let Some(token) = self.client.analytics_token().lock().await.clone() {
            return Ok(token);
        }

        let mut query = HashMap::new();
        query.insert("with_analytics_token".to_string(), "true".to_string());

        let me: Value = self
            .client
            .get("users/@me", Some(query), Some(RequestProperties::home()))
            .await?;

        let token = me
            .get("analytics_token")
            .and_then(|value| value.as_str())
            .ok_or_else(|| {
                Box::<dyn std::error::Error + Send + Sync>::from(
                    "no analytics_token in users/@me response",
                )
            })?
            .to_string();

        *self.client.analytics_token().lock().await = Some(token.clone());
        Ok(token)
    }

    pub async fn post(&self, events: &[ScienceEvent]) -> BoxedResult<()> {
        if events.is_empty() {
            return Ok(());
        }

        let token = self.analytics_token().await?;
        let body = json!({ "token": token, "events": events });

        let _: Value = self
            .client
            .post("science", Some(body), Some(RequestProperties::home()))
            .await?;

        Ok(())
    }
}
