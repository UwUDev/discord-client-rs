use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::channel::Channel;
use discord_client_structs::structs::channel::invite::{CreateChannelInvite, Invite};
use serde_json::{Value, json};

pub struct ChannelRest<'a> {
    pub channel_id: Option<u64>,
    pub guild_id: u64,
    pub client: &'a RestClient,
}

impl<'a> ChannelRest<'a> {
    fn cid(&self) -> BoxedResult<u64> {
        self.channel_id
            .ok_or_else(|| "Channel ID is required".into())
    }

    pub async fn get(&self) -> BoxedResult<Channel> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}", channel_id);

        self.client
            .get::<Channel>(
                &path,
                None,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn trigger_typing(&self) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/typing", channel_id);

        self.client
            .post::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn modify_permissions(&self, overwrite_id: u64, overwrite: Value) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/permissions/{}", channel_id, overwrite_id);

        self.client
            .put::<(), Value>(
                &path,
                Some(overwrite),
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn delete_permission(&self, overwrite_id: u64) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/permissions/{}", channel_id, overwrite_id);

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn follow(&self, webhook_channel_id: u64) -> BoxedResult<Value> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/followers", channel_id);

        let body = json!({ "webhook_channel_id": webhook_channel_id.to_string() });

        self.client
            .post::<Value, Value>(
                &path,
                Some(body),
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn modify_status(&self, status: Option<String>) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/voice-status", channel_id);

        let body = json!({ "status": status });

        self.client
            .put::<(), Value>(
                &path,
                Some(body),
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn get_active_threads(&self) -> BoxedResult<Value> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/threads/active", channel_id);

        self.client
            .get::<Value>(
                &path,
                None,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn get_thread_members(&self) -> BoxedResult<Value> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/thread-members", channel_id);

        self.client
            .get::<Value>(
                &path,
                None,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn join_thread(&self) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/thread-members/@me", channel_id);

        self.client
            .put::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn leave_thread(&self) -> BoxedResult<()> {
        let channel_id = self.cid()?;
        let path = format!("channels/{}/thread-members/@me", channel_id);

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild_channel(self.guild_id, channel_id)),
            )
            .await
    }

    pub async fn create(&self, channel: Channel) -> BoxedResult<Channel> {
        if channel.name.is_none() {
            return Err("Channel name is required".into());
        }

        let path = format!("guilds/{}/channels", self.guild_id);

        let props = RequestProperties::guild(self.guild_id);

        self.client
            .post::<Channel, Channel>(&path, Some(channel), Some(props))
            .await
    }

    pub async fn edit(&self, channel: Channel) -> BoxedResult<Channel> {
        if channel.id == 0 {
            return Err("Channel ID is required".into());
        }

        let path = format!("channels/{}", channel.id);

        let props = RequestProperties::guild_channel(self.guild_id, channel.id);

        self.client
            .patch::<Channel, Channel>(&path, Some(channel), Some(props))
            .await
    }

    pub async fn delete(&self) -> BoxedResult<Channel> {
        if self.channel_id.is_none() {
            return Err("Channel ID is required".into());
        }

        let path = format!("channels/{}", self.channel_id.unwrap());

        let props = RequestProperties::guild_channel(self.guild_id, self.channel_id.unwrap());

        self.client
            .delete::<_, Channel>(&path, None::<Channel>, Some(props))
            .await
    }

    pub async fn create_invite(
        &self,
        create_channel_invite: CreateChannelInvite,
    ) -> BoxedResult<Invite> {
        if self.channel_id.is_none() {
            return Err("Channel ID is required".into());
        }

        let path = format!("channels/{}/invites", self.channel_id.unwrap());

        let props = RequestProperties::guild_channel(self.guild_id, self.channel_id.unwrap());

        self.client
            .post::<Invite, CreateChannelInvite>(&path, Some(create_channel_invite), Some(props))
            .await
    }

    pub async fn get_invites(&self) -> BoxedResult<Vec<Invite>> {
        if self.channel_id.is_none() {
            return Err("Channel ID is required".into());
        }

        let path = format!("channels/{}/invites", self.channel_id.unwrap());

        let props = RequestProperties::guild_channel(self.guild_id, self.channel_id.unwrap());

        self.client
            .get::<Vec<Invite>>(&path, None, Some(props))
            .await
    }
}
