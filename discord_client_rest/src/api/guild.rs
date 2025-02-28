use crate::BoxedResult;
use crate::api::channel::ChannelRest;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, Referer};
use discord_client_structs::structs::guild::log::AuditLog;
use discord_client_structs::structs::guild::log::query::AuditLogQuery;
use discord_client_structs::structs::message::query::{MessageSearchQuery, MessageSearchResult};

pub struct GuildRest<'a> {
    pub guild_id: u64,
    pub client: &'a RestClient,
}

impl<'a> GuildRest<'a> {
    pub fn channel(&self, id: Option<u64>) -> ChannelRest {
        ChannelRest {
            channel_id: id,
            guild_id: self.guild_id,
            client: self.client,
        }
    }

    pub async fn search_guild_messages(
        &self,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        let path = format!("guilds/{}/messages/search", self.guild_id);

        let referer = GuildReferer {
            guild_id: self.guild_id,
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(props))
            .await
    }

    pub async fn get_audit_log(&self, query: AuditLogQuery) -> BoxedResult<AuditLog> {
        let path = format!("guilds/{}/audit-logs", self.guild_id);

        let referer = GuildReferer {
            guild_id: self.guild_id,
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<AuditLog>(&path, Some(query.to_map()), Some(props))
            .await
    }
}
