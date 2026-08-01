use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::user::connection::Connection;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct UserRest<'a> {
    pub client: &'a RestClient,
}

impl<'a> UserRest<'a> {
    pub async fn get_profile(&self, user_id: u64, guild_id: Option<u64>) -> BoxedResult<Value> {
        let path = format!("users/{}/profile", user_id);

        let mut query = HashMap::new();
        query.insert("with_mutual_guilds".to_string(), "true".to_string());
        if let Some(guild_id) = guild_id {
            query.insert("guild_id".to_string(), guild_id.to_string());
        }

        self.client
            .get(&path, Some(query), Some(RequestProperties::home()))
            .await
    }

    pub async fn get_mutual_relationships(&self, user_id: u64) -> BoxedResult<Value> {
        let path = format!("users/{}/relationships", user_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_notes(&self) -> BoxedResult<Value> {
        let path = "users/@me/notes";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_note(&self, user_id: u64) -> BoxedResult<Value> {
        let path = format!("users/@me/notes/{}", user_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn modify_note(&self, user_id: u64, note: String) -> BoxedResult<()> {
        let path = format!("users/@me/notes/{}", user_id);

        let body = json!({ "note": note });

        self.client
            .put::<(), Value>(&path, Some(body), Some(RequestProperties::home()))
            .await
    }

    pub async fn get_recent_mentions(&self, limit: u16) -> BoxedResult<Value> {
        let path = "users/@me/mentions";

        let mut query = HashMap::new();
        query.insert("limit".to_string(), limit.to_string());

        self.client
            .get(&path, Some(query), Some(RequestProperties::home()))
            .await
    }

    pub async fn delete_recent_mention(&self, message_id: u64) -> BoxedResult<()> {
        let path = format!("users/@me/mentions/{}", message_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_connections(&self) -> BoxedResult<Vec<Connection>> {
        let path = "users/@me/connections";

        self.client
            .get::<Vec<Connection>>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_user_affinities(&self) -> BoxedResult<Value> {
        let path = "users/@me/affinities/users";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_guild_affinities(&self) -> BoxedResult<Value> {
        let path = "users/@me/affinities/guilds";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_channel_affinities(&self) -> BoxedResult<Value> {
        let path = "users/@me/affinities/channels";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_tutorial(&self) -> BoxedResult<Value> {
        let path = "tutorial";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_premium_usage(&self) -> BoxedResult<Value> {
        let path = "users/@me/premium-usage";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_saved_messages(&self) -> BoxedResult<Value> {
        let path = "users/@me/saved-messages";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_pomelo_suggestions(&self) -> BoxedResult<Value> {
        let path = "users/@me/pomelo-suggestions";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }

    pub async fn get_survey(&self) -> BoxedResult<Value> {
        let path = "users/@me/survey";

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::home()))
            .await
    }
}
