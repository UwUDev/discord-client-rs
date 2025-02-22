use crate::BoxedResult;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildChannelReferer, GuildReferer, Referer};
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

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .post::<Channel, Channel>(&path, Some(channel), Some(props))
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
        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .patch::<Channel, Channel>(&path, Some(channel), Some(props))
            .await
    }

    pub async fn delete(&self, guild_id: u64, channel_id: u64) -> BoxedResult<Channel> {
        let path = format!("channels/{}", channel_id);

        let referer = GuildChannelReferer {
            guild_id,
            channel_id,
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .delete::<_, Channel>(&path, None::<Channel>, Some(props))
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
        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .post::<Invite, CreateChannelInvite>(&path, Some(create_channel_invite), Some(props))
            .await
    }

    pub async fn get_invites(&self, channel_id: u64, guild_id: u64) -> BoxedResult<Vec<Invite>> {
        let path = format!("channels/{}/invites", channel_id);

        let referer = GuildChannelReferer {
            guild_id,
            channel_id,
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<Vec<Invite>>(&path, None, Some(props))
            .await
    }
}
