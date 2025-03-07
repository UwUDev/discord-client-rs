use crate::BoxedResult;
use crate::captcha::SolvedCaptcha;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::context::Context;
use crate::structs::referer::{HomePageReferer, Referer};
use discord_client_structs::structs::channel::invite::Invite;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct InviteRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> InviteRest<'a> {
    pub async fn get_invite(&self, code: String) -> BoxedResult<Invite> {
        // https://discord.com/api/v9/invites/allah?inputValue=allah&with_counts=true&with_expiration=true&with_permissions=false
        let path = format!("invites/{}", code);

        let referer = HomePageReferer {};

        let mut params = HashMap::new();
        params.insert("inputValue".to_string(), code);
        params.insert("with_counts".to_string(), "true".to_string());
        params.insert("with_expiration".to_string(), "true".to_string());
        params.insert("with_permissions".to_string(), "false".to_string());

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<Invite>(&path, Some(params), Some(props))
            .await
    }

    pub async fn join_invite(
        &self,
        invite: Invite,
        session_id: String,
        solved_captcha: Option<SolvedCaptcha>,
    ) -> BoxedResult<Invite> {
        let path = format!("invites/{}", invite.code);

        let referer = HomePageReferer {};
        let context: Context = invite.into();
        let mut builder = RequestPropertiesBuilder::default();
        let mut props = builder.referer::<Referer>(referer.into()).context(context);

        if let Some(captcha) = solved_captcha {
            props = props.solved_captcha(captcha);
        }

        let payload = json!({
            "session_id": session_id,
        });

        self.client
            .post::<Invite, Value>(&path, Some(payload), Some(props.build().unwrap()))
            .await
    }
}
