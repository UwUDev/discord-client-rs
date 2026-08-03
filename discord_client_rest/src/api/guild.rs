use crate::api::channel::ChannelRest;
use crate::rest::{RequestProperties, RestClient};
use crate::{BoxedResult, MAX_ICON_SIZE};
use discord_client_structs::structs::guild::Guild;
use discord_client_structs::structs::guild::create::CreateGuild;
use discord_client_structs::structs::guild::log::AuditLog;
use discord_client_structs::structs::guild::log::query::AuditLogQuery;
use discord_client_structs::structs::guild::role::Role;
use discord_client_structs::structs::message::query::{MessageSearchQuery, MessageSearchResult};
use discord_client_structs::structs::user::Member;
use serde_json::{Value, json};
use std::collections::HashMap;

pub struct GuildRest<'a> {
    pub guild_id: Option<u64>,
    pub client: &'a RestClient,
}

impl<'a> GuildRest<'a> {
    pub fn channel(&self, id: Option<u64>) -> Option<ChannelRest<'_>> {
        if self.guild_id.is_none() {
            return None;
        }
        Some(ChannelRest {
            channel_id: id,
            guild_id: self.guild_id.unwrap(),
            client: self.client,
        })
    }

    fn gid(&self) -> BoxedResult<u64> {
        self.guild_id.ok_or_else(|| "Guild ID is required".into())
    }

    pub async fn create(&self, create_guild: CreateGuild) -> BoxedResult<Guild> {
        if let Some(icon) = &create_guild.icon {
            if icon.as_bytes().len() > MAX_ICON_SIZE {
                return Err("Encoded icon file is too large".into());
            }
        }

        let path = "guilds";

        let props = RequestProperties::home();

        self.client
            .post::<Guild, CreateGuild>(&path, Some(create_guild), Some(props))
            .await
    }

    pub async fn delete(&self) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}", guild_id);

        self.client
            .delete(&path, None::<&()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get(&self, with_counts: bool) -> BoxedResult<Guild> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}", guild_id);

        let mut query = HashMap::new();
        query.insert("with_counts".to_string(), with_counts.to_string());

        self.client
            .get(&path, Some(query), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_basic(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/basic", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_preview(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/preview", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify(&self, guild: Value) -> BoxedResult<Guild> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}", guild_id);

        self.client
            .patch::<Guild, Value>(&path, Some(guild), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_member(&self, user_id: u64) -> BoxedResult<Member> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/{}", guild_id, user_id);

        self.client
            .get::<Member>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn list_members(
        &self,
        limit: u16,
        after: Option<u64>,
    ) -> BoxedResult<Vec<Member>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members", guild_id);

        let mut query = HashMap::new();
        query.insert("limit".to_string(), limit.to_string());
        if let Some(after) = after {
            query.insert("after".to_string(), after.to_string());
        }

        self.client
            .get::<Vec<Member>>(&path, Some(query), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_current_member(&self) -> BoxedResult<Member> {
        let guild_id = self.gid()?;
        let path = format!("users/@me/guilds/{}/member", guild_id);

        self.client
            .get::<Member>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_member(&self, user_id: u64, member: Value) -> BoxedResult<Member> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/{}", guild_id, user_id);

        self.client
            .patch::<Member, Value>(
                &path,
                Some(member),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn modify_current_member(&self, member: Value) -> BoxedResult<Member> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/@me", guild_id);

        self.client
            .patch::<Member, Value>(
                &path,
                Some(member),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn modify_current_member_nick(&self, nick: String) -> BoxedResult<Member> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/@me/nick", guild_id);

        let body = json!({ "nick": nick });

        self.client
            .patch::<Member, Value>(&path, Some(body), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn add_member_role(&self, user_id: u64, role_id: u64) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);

        self.client
            .put::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn remove_member_role(&self, user_id: u64, role_id: u64) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/{}/roles/{}", guild_id, user_id, role_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn remove_member(&self, user_id: u64) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/members/{}", guild_id, user_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_bans(
        &self,
        limit: u16,
        before: Option<u64>,
        after: Option<u64>,
    ) -> BoxedResult<Vec<Value>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bans", guild_id);

        let mut query = HashMap::new();
        query.insert("limit".to_string(), limit.to_string());
        if let Some(before) = before {
            query.insert("before".to_string(), before.to_string());
        }
        if let Some(after) = after {
            query.insert("after".to_string(), after.to_string());
        }

        self.client
            .get(&path, Some(query), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_ban(&self, user_id: u64) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bans/{}", guild_id, user_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn search_bans(&self, query: String, limit: u16) -> BoxedResult<Vec<Value>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bans/search", guild_id);

        let mut map = HashMap::new();
        map.insert("query".to_string(), query);
        map.insert("limit".to_string(), limit.to_string());

        self.client
            .get(&path, Some(map), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn create_ban(&self, user_id: u64, delete_message_seconds: u32) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bans/{}", guild_id, user_id);

        let body = json!({ "delete_message_seconds": delete_message_seconds });

        self.client
            .put::<(), Value>(&path, Some(body), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn delete_ban(&self, user_id: u64) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bans/{}", guild_id, user_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn bulk_ban(
        &self,
        user_ids: Vec<u64>,
        delete_message_seconds: u32,
    ) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/bulk-ban", guild_id);

        let body = json!({
            "user_ids": user_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
            "delete_message_seconds": delete_message_seconds,
        });

        self.client
            .post::<Value, Value>(&path, Some(body), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_roles(&self) -> BoxedResult<Vec<Role>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles", guild_id);

        self.client
            .get::<Vec<Role>>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_role(&self, role_id: u64) -> BoxedResult<Role> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/{}", guild_id, role_id);

        self.client
            .get::<Role>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_role_member_counts(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/member-counts", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_role_member_ids(&self, role_id: u64) -> BoxedResult<Vec<String>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/{}/member-ids", guild_id, role_id);

        self.client
            .get::<Vec<String>>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn create_role(&self, role: Value) -> BoxedResult<Role> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles", guild_id);

        self.client
            .post::<Role, Value>(&path, Some(role), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_role(&self, role_id: u64, role: Value) -> BoxedResult<Role> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/{}", guild_id, role_id);

        self.client
            .patch::<Role, Value>(&path, Some(role), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_role_positions(&self, positions: Value) -> BoxedResult<Vec<Role>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles", guild_id);

        self.client
            .patch::<Vec<Role>, Value>(
                &path,
                Some(positions),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn add_role_members(&self, role_id: u64, member_ids: Vec<u64>) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/{}/members", guild_id, role_id);

        let body = json!({
            "member_ids": member_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>(),
        });

        self.client
            .patch::<Value, Value>(&path, Some(body), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn delete_role(&self, role_id: u64) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/roles/{}", guild_id, role_id);

        self.client
            .delete::<(), ()>(&path, None::<()>, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_prune(&self, days: u8) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/prune", guild_id);

        let mut query = HashMap::new();
        query.insert("days".to_string(), days.to_string());

        self.client
            .get(&path, Some(query), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn prune(&self, days: u8, compute_prune_count: bool) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/prune", guild_id);

        let body = json!({
            "days": days,
            "compute_prune_count": compute_prune_count,
        });

        self.client
            .post::<Value, Value>(&path, Some(body), Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_vanity_invite(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/vanity-url", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_widget_settings(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/widget", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_widget(&self, widget: Value) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/widget", guild_id);

        self.client
            .patch::<Value, Value>(
                &path,
                Some(widget),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn get_widget(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/widget.json", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_welcome_screen(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/welcome-screen", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_welcome_screen(&self, welcome_screen: Value) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/welcome-screen", guild_id);

        self.client
            .patch::<Value, Value>(
                &path,
                Some(welcome_screen),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn get_onboarding(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/onboarding", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_new_member_welcome(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/new-member-welcome", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_user_guilds(client: &RestClient) -> BoxedResult<Value> {
        let path = "users/@me/guilds";

        let props = RequestProperties::home();

        client.get::<Value>(&path, None, Some(props)).await
    }

    pub async fn get_join_request_guilds(client: &RestClient) -> BoxedResult<Value> {
        let path = "users/@me/join-request-guilds";

        let props = RequestProperties::home();

        client.get::<Value>(&path, None, Some(props)).await
    }

    pub async fn get_member_verification(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/member-verification", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_channels(
        &self,
    ) -> BoxedResult<Vec<discord_client_structs::structs::channel::Channel>> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/channels", guild_id);

        self.client
            .get(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn get_active_threads(&self) -> BoxedResult<Value> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/threads/active", guild_id);

        self.client
            .get::<Value>(&path, None, Some(RequestProperties::guild(guild_id)))
            .await
    }

    pub async fn modify_channel_positions(&self, positions: Value) -> BoxedResult<()> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/channels", guild_id);

        self.client
            .patch::<(), Value>(
                &path,
                Some(positions),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn leave(&self) -> BoxedResult<()> {
        if self.guild_id.is_none() {
            return Err("Guild ID is required".into());
        }

        let path = format!("users/@me/guilds/{}", self.guild_id.unwrap());

        let props = RequestProperties::guild(self.guild_id.unwrap());

        self.client.delete(&path, None::<&()>, Some(props)).await
    }

    pub async fn search_guild_messages(
        &self,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/messages/search", guild_id);

        self.client
            .get::<MessageSearchResult>(
                &path,
                Some(query.to_map()),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }

    pub async fn get_audit_log(&self, query: AuditLogQuery) -> BoxedResult<AuditLog> {
        let guild_id = self.gid()?;
        let path = format!("guilds/{}/audit-logs", guild_id);

        self.client
            .get::<AuditLog>(
                &path,
                Some(query.to_map()),
                Some(RequestProperties::guild(guild_id)),
            )
            .await
    }
}
