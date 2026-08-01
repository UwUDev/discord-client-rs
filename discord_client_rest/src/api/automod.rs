use crate::BoxedResult;
use crate::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::guild::automod::AutomodRule;
use serde_json::Value;

pub struct AutomodRest<'a> {
    pub guild_id: u64,
    pub client: &'a RestClient,
}

impl<'a> AutomodRest<'a> {
    pub async fn get_rules(&self) -> BoxedResult<Vec<AutomodRule>> {
        let path = format!("guilds/{}/auto-moderation/rules", self.guild_id);

        self.client
            .get::<Vec<AutomodRule>>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn get_rule(&self, rule_id: u64) -> BoxedResult<AutomodRule> {
        let path = format!("guilds/{}/auto-moderation/rules/{}", self.guild_id, rule_id);

        self.client
            .get::<AutomodRule>(&path, None, Some(RequestProperties::guild(self.guild_id)))
            .await
    }

    pub async fn create_rule(&self, rule: Value) -> BoxedResult<AutomodRule> {
        let path = format!("guilds/{}/auto-moderation/rules", self.guild_id);

        self.client
            .post::<AutomodRule, Value>(
                &path,
                Some(rule),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn modify_rule(&self, rule_id: u64, rule: Value) -> BoxedResult<AutomodRule> {
        let path = format!("guilds/{}/auto-moderation/rules/{}", self.guild_id, rule_id);

        self.client
            .patch::<AutomodRule, Value>(
                &path,
                Some(rule),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn delete_rule(&self, rule_id: u64) -> BoxedResult<()> {
        let path = format!("guilds/{}/auto-moderation/rules/{}", self.guild_id, rule_id);

        self.client
            .delete::<(), ()>(
                &path,
                None::<()>,
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }

    pub async fn validate_rule(&self, rule: Value) -> BoxedResult<Value> {
        let path = format!("guilds/{}/auto-moderation/rules/validate", self.guild_id);

        self.client
            .post::<Value, Value>(
                &path,
                Some(rule),
                Some(RequestProperties::guild(self.guild_id)),
            )
            .await
    }
}
