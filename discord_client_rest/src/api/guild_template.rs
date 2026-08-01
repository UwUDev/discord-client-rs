use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::guild::Guild;
use serde_json::Value;

pub struct GuildTemplateRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> GuildTemplateRest<'a> {
    pub async fn get_all(&self, guild_id: u64) -> BoxedResult<Value> {
        let path = format!("guilds/{}/templates", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get(&self, code: String) -> BoxedResult<Value> {
        let path = format!("guilds/templates/{}", code);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn create(&self, guild_id: u64, template: Value) -> BoxedResult<Value> {
        let path = format!("guilds/{}/templates", guild_id);

        self.client
            .post::<Value, Value>(
                &path,
                Some(template),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn modify(&self, guild_id: u64, code: String, template: Value) -> BoxedResult<Value> {
        let path = format!("guilds/{}/templates/{}", guild_id, code);

        self.client
            .patch::<Value, Value>(
                &path,
                Some(template),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn sync(&self, guild_id: u64, code: String) -> BoxedResult<Value> {
        let path = format!("guilds/{}/templates/{}", guild_id, code);

        self.client
            .put::<Value, ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn delete(&self, guild_id: u64, code: String) -> BoxedResult<Value> {
        let path = format!("guilds/{}/templates/{}", guild_id, code);

        self.client
            .delete::<Value, ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn use_template(&self, code: String, guild: Value) -> BoxedResult<Guild> {
        let path = format!("guilds/templates/{}", code);

        self.client
            .post::<Guild, Value>(&path, Some(guild), Some(RequestProperties::home()))
            .await
    }
}
