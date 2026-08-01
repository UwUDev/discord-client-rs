use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use serde_json::Value;

pub struct StickerRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> StickerRest<'a> {
    pub async fn get_packs(&self) -> BoxedResult<Value> {
        let path = "sticker-packs";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_pack(&self, pack_id: u64) -> BoxedResult<Value> {
        let path = format!("sticker-packs/{}", pack_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get(&self, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("stickers/{}", sticker_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_sticker_guild(&self, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("stickers/{}/guild", sticker_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_guild_stickers(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/stickers", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_guild_sticker(&self, guild_id: u64, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/stickers/{}", guild_id, sticker_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_guild_sticker(
        &self,
        guild_id: u64,
        sticker_id: u64,
        sticker: Value,
    ) -> BoxedResult<Value> {
        let path = format!("guilds/{}/stickers/{}", guild_id, sticker_id);

        self.client
            .patch::<Value, Value>(
                &path,
                Some(sticker),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn delete_guild_sticker(&self, guild_id: u64, sticker_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/stickers/{}", guild_id, sticker_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }
}
