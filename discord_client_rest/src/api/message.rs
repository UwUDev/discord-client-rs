use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use crate::structs::referer::{DmChannelReferer, GuildChannelReferer, Referer};
use discord_client_structs::structs::message::Message;
use discord_client_structs::structs::message::query::{
    MessageQuery, MessageSearchQuery, MessageSearchResult,
};
use serde_json::{Value, json};
use std::collections::HashMap;

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

        let props = RequestProperties::from_referer(referer);

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

        let props = RequestProperties::from_referer(referer);

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

        let props = RequestProperties::from_referer(referer);

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

        let props = RequestProperties::from_referer(referer);

        self.client
            .get::<Vec<Message>>(&path, Some(query.to_map()), Some(props))
            .await
    }

    pub async fn search_dm_messages(
        &self,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let path = format!("channels/{}/messages/search", self.channel_id);

        let props = RequestProperties::dm_channel(self.channel_id);

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

        let props = RequestProperties::from_referer(referer);

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

        let props = RequestProperties::from_referer(referer);

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

        let props = RequestProperties::from_referer(referer);

        self.client
            .get::<Vec<Message>>(&path, None, Some(props))
            .await
    }

    pub async fn add_reaction(
        &self,
        message_id: u64,
        emoji: String,
        burst: bool,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let mut path = format!(
            "channels/{}/messages/{}/reactions/{}/@me",
            self.channel_id, message_id, emoji
        );
        if burst {
            path.push_str("?type=1");
        }

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

        let props = RequestProperties::from_referer(referer);

        self.client
            .put::<_, ()>(&path, None::<()>, Some(props))
            .await
    }

    fn referer(&self, guild_id: Option<u64>) -> Referer {
        match guild_id {
            Some(guild_id) => GuildChannelReferer {
                guild_id,
                channel_id: self.channel_id,
            }
            .into(),
            None => DmChannelReferer {
                channel_id: self.channel_id,
            }
            .into(),
        }
    }

    pub async fn get_reactions(
        &self,
        message_id: u64,
        emoji: String,
        burst: bool,
        limit: u16,
        after: Option<u64>,
        guild_id: Option<u64>,
    ) -> BoxedResult<Vec<discord_client_structs::structs::user::User>> {
        let path = format!(
            "channels/{}/messages/{}/reactions/{}",
            self.channel_id, message_id, emoji
        );

        let mut query = HashMap::new();
        query.insert(
            "type".to_string(),
            if burst { "1" } else { "0" }.to_string(),
        );
        query.insert("limit".to_string(), limit.to_string());
        if let Some(after) = after {
            query.insert("after".to_string(), after.to_string());
        }

        let props = RequestProperties::from_referer(self.referer(guild_id));

        self.client.get(&path, Some(query), Some(props)).await
    }

    pub async fn delete_all_reactions(
        &self,
        message_id: u64,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let path = format!(
            "channels/{}/messages/{}/reactions",
            self.channel_id, message_id
        );

        let props = RequestProperties::from_referer(self.referer(guild_id));

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn delete_all_reactions_for_emoji(
        &self,
        message_id: u64,
        emoji: String,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let path = format!(
            "channels/{}/messages/{}/reactions/{}",
            self.channel_id, message_id, emoji
        );

        let props = RequestProperties::from_referer(self.referer(guild_id));

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn crosspost(&self, message_id: u64, guild_id: u64) -> BoxedResult<Message> {
        let path = format!(
            "channels/{}/messages/{}/crosspost",
            self.channel_id, message_id
        );

        let props = RequestProperties::from_referer(self.referer(Some(guild_id)));

        self.client
            .post::<Message, ()>(&path, None::<()>, Some(props))
            .await
    }

    pub async fn bulk_delete(&self, message_ids: Vec<u64>, guild_id: u64) -> BoxedResult<()> {
        let path = format!("channels/{}/messages/bulk-delete", self.channel_id);

        let body = json!({
            "messages": message_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
        });

        let props = RequestProperties::from_referer(self.referer(Some(guild_id)));

        self.client
            .post::<(), Value>(&path, Some(body), Some(props))
            .await
    }

    pub async fn acknowledge(&self, message_id: u64, guild_id: Option<u64>) -> BoxedResult<Value> {
        let path = format!("channels/{}/messages/{}/ack", self.channel_id, message_id);

        let body = json!({ "token": Value::Null });

        let props = RequestProperties::from_referer(self.referer(guild_id));

        self.client
            .post::<Value, Value>(&path, Some(body), Some(props))
            .await
    }

    pub async fn remove_reaction(
        &self,
        message_id: u64,
        emoji: String,
        user_id: Option<u64>,
        burst: bool,
        guild_id: Option<u64>,
    ) -> BoxedResult<()> {
        let reaction_type = if burst { 1 } else { 0 };
        let reaction_user = match user_id {
            Some(user_id) => user_id.to_string(),
            None => String::from("@me"),
        };

        let path = format!(
            "channels/{}/messages/{}/reactions/{}/{}/{}",
            self.channel_id, message_id, emoji, reaction_type, reaction_user
        );

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

        let props = RequestProperties::from_referer(referer);

        self.client
            .delete::<_, ()>(&path, None::<()>, Some(props))
            .await
    }
}
