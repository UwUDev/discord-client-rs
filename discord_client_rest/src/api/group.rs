use crate::BoxedResult;
use crate::rest::RestClient;
use crate::structs::context::Context::NewGroupDmContext;
use crate::structs::referer::{DmChannelReferer, HomePageReferer};
use discord_client_structs::structs::channel::Channel;
use serde_json::{Value, json};

pub struct GroupRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> GroupRest<'a> {
    pub async fn create_group(&self, user_ids: Vec<u64>) -> BoxedResult<Channel> {
        let path = String::from("users/@me/channels");

        let payload = json!({
          "recipients": user_ids.iter().map(|id| id.to_string()).collect::<Vec<String>>(),
        });

        let context = NewGroupDmContext;

        let referer = HomePageReferer;

        self.client
            .post::<Channel, Value>(&path, Some(payload), Some(referer.into()), Some(context))
            .await
    }

    pub async fn add_user_to_group(&self, channel_id: u64, user_id: u64) -> BoxedResult<()> {
        let path = format!("users/@me/channels/{}/recipients/{}", channel_id, user_id);

        let referer = DmChannelReferer { channel_id };

        self.client
            .put::<_, _>(&path, None::<&()>, Some(referer.into()), None)
            .await
    }

    pub async fn kick_user_from_group(&self, channel_id: u64, user_id: u64) -> BoxedResult<()> {
        let path = format!("channels/{}/recipients/{}", channel_id, user_id);

        let referer = DmChannelReferer { channel_id };

        self.client
            .delete::<_, _>(&path, None::<&()>, Some(referer.into()), None)
            .await
    }

    pub async fn transfer_group_ownership(
        &self,
        channel_id: u64,
        user_id: u64,
    ) -> BoxedResult<Channel> {
        let path = format!("channels/{}", channel_id);

        let referer = DmChannelReferer { channel_id };

        let payload = json!({
          "owner": user_id.to_string(),
        });

        self.client
            .put::<Channel, _>(&path, Some(payload), Some(referer.into()), None)
            .await
    }

    pub async fn rename_group(&self, channel_id: u64, name: &str) -> BoxedResult<Channel> {
        let path = format!("channels/{}", channel_id);

        let referer = DmChannelReferer { channel_id };

        let payload = json!({
          "name": name,
        });

        self.client
            .patch::<Channel, _>(&path, Some(payload), Some(referer.into()), None)
            .await
    }

    // todo: set group icon

    pub async fn leave_group(&self, channel_id: u64, silent: bool) -> BoxedResult<Channel> {
        let path = format!("channels/{}?silent={}", channel_id, silent);

        let referer = DmChannelReferer { channel_id };

        self.client
            .delete::<Channel, _>(&path, None::<&()>, Some(referer.into()), None)
            .await
    }
}
