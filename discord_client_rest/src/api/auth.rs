use crate::BoxedResult;
use crate::mfa::MfaType;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{HomePageReferer, Referer};
use serde_json::{Value, json};

pub struct AuthRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> AuthRest<'a> {
    pub async fn validate_mfa(
        &self,
        data: String,
        r#type: MfaType,
        ticket: String,
    ) -> BoxedResult<Value> {
        let path = String::from("mfa/finish");

        let payload = json!({
            "data": data,
            "mfa_type": r#type.to_string(),
            "ticket": ticket,
        });

        let referer = HomePageReferer {};

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .post::<Value, Value>(&path, Some(payload), Some(props))
            .await
    }
}
