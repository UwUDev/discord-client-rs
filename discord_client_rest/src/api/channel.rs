use crate::BoxedResult;
use crate::rest::RestClient;
use crate::structs::referer::{GuildChannelReferer, GuildReferer};
use discord_client_structs::structs::channel::Channel;
use discord_client_structs::structs::channel::invite::{CreateChannelInvite, Invite};

pub struct ChannelRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> ChannelRest<'a> {
    pub async fn create(&self, guild_id: u64, channel: Channel) -> BoxedResult<Channel> {
        if channel.name.is_none() {
            return Err("Channel name is required".into());
        }

        let path = format!("guilds/{}/channels", guild_id);
        let referer = GuildReferer { guild_id };
        self.client
            .post::<Channel, Channel>(&path, Some(channel), Some(referer.into()))
            .await
    }

    pub async fn edit(&self, guild_id: u64, channel: Channel) -> BoxedResult<Channel> {
        if channel.id == 0 {
            return Err("Channel ID is required".into());
        }

        let path = format!("channels/{}", channel.id);
        let referer = GuildChannelReferer {
            guild_id,
            channel_id: channel.id,
        };
        self.client
            .patch::<Channel, Channel>(&path, Some(channel), Some(referer.into()))
            .await
    }

    pub async fn delete(&self, guild_id: u64, channel_id: u64) -> BoxedResult<Channel> {
        let path = format!("channels/{}", channel_id);
        let referer = GuildChannelReferer {
            guild_id,
            channel_id,
        };
        self.client
            .delete::<_, Channel>(&path, None::<Channel>, Some(referer.into()))
            .await
    }

    pub async fn create_invite(
        &self,
        channel_id: u64,
        guild_id: u64,
        create_channel_invite: CreateChannelInvite,
    ) -> BoxedResult<Invite> {
        let path = format!("channels/{}/invites", channel_id);
        let referer = GuildChannelReferer {
            guild_id,
            channel_id,
        };
        self.client
            .post::<Invite, CreateChannelInvite>(
                &path,
                Some(create_channel_invite),
                Some(referer.into()),
            )
            .await
    }

    pub async fn get_invites(&self, channel_id: u64, guild_id: u64) -> BoxedResult<Vec<Invite>> {
        let path = format!("channels/{}/invites", channel_id);
        let referer = GuildChannelReferer {
            guild_id,
            channel_id,
        };
        self.client
            .get::<Vec<Invite>>(&path, None, Some(referer.into()))
            .await
    }
}
