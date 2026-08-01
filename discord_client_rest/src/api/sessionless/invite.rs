use crate::BoxedResult;
use crate::rest::RequestPropertiesBuilder;
use crate::sessionless::SessionlessClient;
use crate::structs::referer::{HomePageReferer, Referer};
use discord_client_structs::structs::channel::invite::Invite;
use std::collections::HashMap;

pub struct SessionlessInviteRest<'a> {
    pub client: &'a SessionlessClient,
}

impl<'a> SessionlessInviteRest<'a> {
    pub async fn get_invite(&self, code: String) -> BoxedResult<Invite> {
        let path = format!("invites/{}", code);

        let referer = HomePageReferer {};

        let mut params = HashMap::new();
        params.insert("with_counts".to_string(), "true".to_string());
        params.insert("with_expiration".to_string(), "true".to_string());
        params.insert("with_permissions".to_string(), "true".to_string());
        params.insert("with_games".to_string(), "true".to_string());

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<Invite>(&path, Some(params), Some(props))
            .await
    }
}
