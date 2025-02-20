use crate::BoxedResult;
use crate::rest::RestClient;
use crate::structs::referer::{DmChannelReferer, GuildChannelReferer, GuildReferer, Referer};
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::message::query::{
    MessageQuery, MessageSearchQuery, MessageSearchResult,
};

pub struct MessageRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> MessageRest<'a> {
    pub async fn send(
        &self,
        channel_id: u64,
        guild_id: Option<u64>,
        message: Message,
    ) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages", channel_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .post::<Message, Message>(&path, Some(message), Some(referer.into()))
            .await
    }

    pub async fn edit(
        &self,
        channel_id: u64,
        message_id: u64,
        guild_id: Option<u64>,
        message: Message,
    ) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages/{}", channel_id, message_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .patch::<Message, Message>(&path, Some(message), Some(referer.into()))
            .await
    }

    pub async fn delete(
        &self,
        channel_id: u64,
        message_id: u64,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let path = format!("channels/{}/messages/{}", channel_id, message_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .delete::<_, ()>(&path, None::<()>, Some(referer.into()))
            .await
    }

    pub async fn get_channel_messages(
        &self,
        channel_id: u64,
        guild_id: Option<u64>,
        query: MessageQuery,
    ) -> BoxedResult<Vec<Message>> {
        let path = format!("channels/{}/messages", channel_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .get::<Vec<Message>>(&path, Some(query.to_map()), Some(referer.into()))
            .await
    }

    pub async fn search_dm_messages(
        &self,
        channel_id: u64,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let path = format!("channels/{}/messages/search", channel_id);
        let referer = DmChannelReferer { channel_id };
        self.client
            .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(referer.into()))
            .await
    }

    pub async fn search_guild_messages(
        &self,
        guild_id: u64,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let path = format!("guilds/{}/messages/search", guild_id);
        let referer = GuildReferer { guild_id };
        self.client
            .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(referer.into()))
            .await
    }

    pub async fn pin(
        &self,
        channel_id: u64,
        message_id: u64,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let path = format!("channels/{}/pins/{}", channel_id, message_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .put::<_, ()>(&path, None::<()>, Some(referer.into()))
            .await
    }

    pub async fn unpin(
        &self,
        channel_id: u64,
        message_id: u64,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let path = format!("channels/{}/pins/{}", channel_id, message_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .delete::<_, ()>(&path, None::<()>, Some(referer.into()))
            .await
    }

    pub async fn get_pinned_messages(
        &self,
        channel_id: u64,
        guild_id: Option<u64>,
    ) -> BoxedResult<Vec<Message>> {
        let path = format!("channels/{}/pins", channel_id);
        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };
        self.client
            .get::<Vec<Message>>(&path, None, Some(referer.into()))
            .await
    }
}
