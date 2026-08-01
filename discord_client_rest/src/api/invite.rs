use crate::BoxedResult;
use crate::captcha::SolvedCaptcha;
use crate::rest::{RequestProperties, RestClient};
use crate::structs::context::Context;
use discord_client_structs::structs::channel::invite::Invite;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct InviteRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> InviteRest<'a> {
    pub async fn get_invite(&self, code: String) -> BoxedResult<Invite> {
        let path = format!("invites/{}", code);

        let mut params = HashMap::new();
        params.insert("with_counts".to_string(), "true".to_string());
        params.insert("with_expiration".to_string(), "true".to_string());
        params.insert("with_permissions".to_string(), "true".to_string());
        params.insert("with_games".to_string(), "true".to_string());

        let props = RequestProperties::home();

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

        let context: Context = invite.into();
        let mut props = RequestProperties::home().with_context(context);

        if let Some(captcha) = solved_captcha {
            props = props.with_solved_captcha(captcha);
        }

        let payload = json!({
            "session_id": session_id,
        });

        self.client
            .post::<Invite, Value>(&path, Some(payload), Some(props))
            .await
    }
}
