use crate::BoxedResult;
use crate::rest::RestClient;
use crate::structs::context::Context::NoContext;
use crate::structs::referer::{HomePageReferer, Referer};
use discord_client_structs::structs::channel::Channel;
use serde_json::{Value, json};

pub struct DmRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> DmRest<'a> {
    // todo: handle captcha
    pub async fn open_or_create_dm_channel(
        &self,
        user_id: u64,
        referer: Referer,
    ) -> BoxedResult<Channel> {
        let path = String::from("users/@me/channels");

        let payload = json!({
          "recipients": [
            user_id.to_string()
          ]
        });

        let context = NoContext;

        self.client
            .post::<Channel, Value>(&path, Some(payload), Some(referer.into()), Some(context))
            .await
    }

    pub async fn get_dm_channels(&self) -> BoxedResult<Vec<Channel>> {
        let path = String::from("users/@me/channels");

        let referer = HomePageReferer;

        self.client
            .get::<Vec<Channel>>(&path, None, Some(referer.into()), None)
            .await
    }

    pub async fn close_dm_channel(&self, channel_id: u64) -> BoxedResult<Channel> {
        let group_rest = self.client.group();
        group_rest.leave_group(channel_id, false).await
    }
}
