use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::misc::Emoji;
use serde_json::Value;

pub struct EmojiRest<'a> {
    pub guild_id: u64,
    pub client: &'a RestClient,
}

impl<'a> EmojiRest<'a> {
    pub async fn get_all(&self) -> BoxedResult<Vec<Emoji>> {
        let path = format!("guilds/{}/emojis", self.guild_id);

        self.client
            .get::<Vec<Emoji>>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn get(&self, emoji_id: u64) -> BoxedResult<Emoji> {
        let path = format!("guilds/{}/emojis/{}", self.guild_id, emoji_id);

        self.client
            .get::<Emoji>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn get_top(&self) -> BoxedResult<Value> {
        let path = format!("guilds/{}/top-emojis", self.guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn create(&self, emoji: Value) -> BoxedResult<Emoji> {
        let path = format!("guilds/{}/emojis", self.guild_id);

        self.client
            .post::<Emoji, Value>(
                &path,
                Some(emoji),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn modify(&self, emoji_id: u64, emoji: Value) -> BoxedResult<Emoji> {
        let path = format!("guilds/{}/emojis/{}", self.guild_id, emoji_id);

        self.client
            .patch::<Emoji, Value>(
                &path,
                Some(emoji),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn delete(&self, emoji_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/emojis/{}", self.guild_id, emoji_id);

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn get_emoji_guild(&self, emoji_id: u64) -> BoxedResult<Value> {
        let path = format!("emojis/{}/guild", emoji_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }
}
