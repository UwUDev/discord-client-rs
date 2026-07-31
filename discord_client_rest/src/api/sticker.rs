use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, HomePageReferer, Referer};
use serde_json::Value;

pub struct StickerRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> StickerRest<'a> {
    fn home_props(&self) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(HomePageReferer {}.into())
            .build()?)
    }

    fn guild_props(&self, guild_id: u64) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(GuildReferer { guild_id }.into())
            .build()?)
    }

    pub async fn get_packs(&self) -> BoxedResult<Value> {
        let path = "sticker-packs";

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_pack(&self, pack_id: u64) -> BoxedResult<Value> {
        let path = format!("sticker-packs/{}", pack_id);

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get(&self, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("stickers/{}", sticker_id);

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_sticker_guild(&self, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("stickers/{}/guild", sticker_id);

        self.client
            .get::<Value>(&path, None, Some(self.home_props()?))
            .await
    }

    pub async fn get_guild_stickers(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/stickers", guild_id);

        self.client
            .get::<Value>(&path, None, Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn get_guild_sticker(&self, guild_id: u64, sticker_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/stickers/{}", guild_id, sticker_id);

        self.client
            .get::<Value>(&path, None, Some(self.guild_props(guild_id)?))
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
            .patch::<Value, Value>(&path, Some(sticker), Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn delete_guild_sticker(&self, guild_id: u64, sticker_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/stickers/{}", guild_id, sticker_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.guild_props(guild_id)?))
            .await
    }
}
