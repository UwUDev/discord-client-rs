use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::guild::event::GuildScheduledEvent;
use serde_json::Value;
use std::collections::HashMap;

pub struct ScheduledEventRest<'a> {
    pub guild_id: u64,
    pub client: &'a RestClient,
}

impl<'a> ScheduledEventRest<'a> {
    pub async fn get_all(&self, with_user_count: bool) -> BoxedResult<Vec<GuildScheduledEvent>> {
        let path = format!("guilds/{}/scheduled-events", self.guild_id);

        let mut query = HashMap::new();
        query.insert("with_user_count".to_string(), with_user_count.to_string());

        self.client
            .get(
                &path,
                Some(query),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn get(
        &self,
        event_id: u64,
        with_user_count: bool,
    ) -> BoxedResult<GuildScheduledEvent> {
        let path = format!("guilds/{}/scheduled-events/{}", self.guild_id, event_id);

        let mut query = HashMap::new();
        query.insert("with_user_count".to_string(), with_user_count.to_string());

        self.client
            .get(
                &path,
                Some(query),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn create(&self, event: Value) -> BoxedResult<GuildScheduledEvent> {
        let path = format!("guilds/{}/scheduled-events", self.guild_id);

        self.client
            .post::<GuildScheduledEvent, Value>(
                &path,
                Some(event),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn modify(&self, event_id: u64, event: Value) -> BoxedResult<GuildScheduledEvent> {
        let path = format!("guilds/{}/scheduled-events/{}", self.guild_id, event_id);

        self.client
            .patch::<GuildScheduledEvent, Value>(
                &path,
                Some(event),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn delete(&self, event_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/scheduled-events/{}", self.guild_id, event_id);

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn get_user_count(&self, event_id: u64) -> BoxedResult<Value> {
        let path = format!(
            "guilds/{}/scheduled-events/{}/users/counts",
            self.guild_id, event_id
        );

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn get_users(&self, event_id: u64, limit: u16) -> BoxedResult<Value> {
        let path = format!(
            "guilds/{}/scheduled-events/{}/users",
            self.guild_id, event_id
        );

        let mut query = HashMap::new();
        query.insert("limit".to_string(), limit.to_string());

        self.client
            .get(
                &path,
                Some(query),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn add_current_user(&self, event_id: u64) -> BoxedResult<Value> {
        let path = format!(
            "guilds/{}/scheduled-events/{}/users/@me",
            self.guild_id, event_id
        );

        self.client
            .put::<Value, ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn remove_current_user(&self, event_id: u64) -> BoxedResult<()> {
        let path = format!(
            "guilds/{}/scheduled-events/{}/users/@me",
            self.guild_id, event_id
        );

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn get_for_current_user(
        client: &RestClient,
    ) -> BoxedResult<Vec<GuildScheduledEvent>> {
        let path = "users/@me/scheduled-events";

        let props = RequestProperties::home();

        client.get(&path, None, Some(props)).await
    }
}
