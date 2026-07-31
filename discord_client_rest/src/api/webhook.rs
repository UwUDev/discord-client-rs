use crate::BoxedResult;
use crate::rest::{RequestProperties, RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildChannelReferer, GuildReferer, Referer};
use discord_client_structs::structs::channel::webhook::Webhook;
use serde_json::Value;

pub struct WebhookRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> WebhookRest<'a> {
    pub async fn get_channel_webhooks(
        &self,
        channel_id: u64,
        guild_id: u64,
    ) -> BoxedResult<Vec<Webhook>> {
        let path = format!("channels/{}/webhooks", channel_id);

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(
                GuildChannelReferer {
                    guild_id,
                    channel_id,
                }
                .into(),
            )
            .build()?;

        self.client
            .get::<Vec<Webhook>>(&path, None, Some(props))
            .await
    }

    pub async fn get_guild_webhooks(&self, guild_id: u64) -> BoxedResult<Vec<Webhook>> {
        let path = format!("guilds/{}/webhooks", guild_id);

        let props = self.guild_props(guild_id)?;

        self.client
            .get::<Vec<Webhook>>(&path, None, Some(props))
            .await
    }

    pub async fn get(&self, webhook_id: u64) -> BoxedResult<Webhook> {
        let path = format!("webhooks/{}", webhook_id);

        self.client.get::<Webhook>(&path, None, None).await
    }

    pub async fn create(
        &self,
        channel_id: u64,
        guild_id: u64,
        webhook: Value,
    ) -> BoxedResult<Webhook> {
        let path = format!("channels/{}/webhooks", channel_id);

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(
                GuildChannelReferer {
                    guild_id,
                    channel_id,
                }
                .into(),
            )
            .build()?;

        self.client
            .post::<Webhook, Value>(&path, Some(webhook), Some(props))
            .await
    }

    pub async fn modify(
        &self,
        webhook_id: u64,
        guild_id: u64,
        webhook: Value,
    ) -> BoxedResult<Webhook> {
        let path = format!("webhooks/{}", webhook_id);

        self.client
            .patch::<Webhook, Value>(&path, Some(webhook), Some(self.guild_props(guild_id)?))
            .await
    }

    pub async fn delete(&self, webhook_id: u64, guild_id: u64) -> BoxedResult<()> {
        let path = format!("webhooks/{}", webhook_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(self.guild_props(guild_id)?))
            .await
    }

    fn guild_props(&self, guild_id: u64) -> BoxedResult<RequestProperties> {
        Ok(RequestPropertiesBuilder::default()
            .referer::<Referer>(GuildReferer { guild_id }.into())
            .build()?)
    }
}
