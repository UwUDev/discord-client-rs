use crate::BoxedResult;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{DmChannelReferer, GuildChannelReferer, Referer};
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::message::query::{
    MessageQuery, MessageSearchQuery, MessageSearchResult,
};

pub struct MessageRest<'a> {
    pub channel_id: u64,
    pub client: &'a RestClient,
}

impl<'a> MessageRest<'a> {
    pub async fn send(&self, guild_id: Option<u64>, message: Message) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages", self.channel_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .post::<Message, Message>(&path, Some(message), Some(props))
            .await
    }

    pub async fn edit(
        &self,
        message_id: u64,
        guild_id: Option<u64>,
        message: Message,
    ) -> BoxedResult<Message> {
        let path = format!("channels/{}/messages/{}", self.channel_id, message_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .patch::<Message, Message>(&path, Some(message), Some(props))
            .await
    }

    pub async fn delete(&self, channel_id: u64, guild_id: Option<u64>) -> BoxedResult<()> {
        let path = format!("channels/{}/messages/{}", self.channel_id, channel_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
            None => DmChannelReferer { channel_id }.into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .delete::<_, ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn get_channel_messages(
        &self,
        guild_id: Option<u64>,
        query: MessageQuery,
    ) -> BoxedResult<Vec<Message>> {
        let path = format!("channels/{}/messages", self.channel_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<Vec<Message>>(&path, Some(query.to_map()), Some(props))
            .await
    }

    pub async fn search_dm_messages(
        &self,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let path = format!("channels/{}/messages/search", self.channel_id);

        let referer = DmChannelReferer {
            channel_id: self.channel_id,
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(props))
            .await
    }

    pub async fn pin(&self, message_id: u64, guild_id: Option<u64>) -> BoxedResult<()> {
        let path = format!("channels/{}/pins/{}", self.channel_id, message_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .put::<_, ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn unpin(&self, message_id: u64, guild_id: Option<u64>) -> BoxedResult<()> {
        let path = format!("channels/{}/pins/{}", self.channel_id, message_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .delete::<_, ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn get_pinned_messages(&self, guild_id: Option<u64>) -> BoxedResult<Vec<Message>> {
        let path = format!("channels/{}/pins", self.channel_id);

        let referer: Referer = match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<Vec<Message>>(&path, None, Some(props))
            .await
    }
}
