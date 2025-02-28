use crate::api::channel::ChannelRest;
use crate::rest::{RequestPropertiesBuilder, RestClient};
use crate::structs::referer::{GuildReferer, HomePageReferer, Referer};
use crate::{BoxedResult, MAX_ICON_SIZE};
use discord_client_structs::structs::guild::create::CreateGuild;
use discord_client_structs::structs::guild::log::query::AuditLogQuery;
use discord_client_structs::structs::guild::log::AuditLog;
use discord_client_structs::structs::guild::Guild;
use discord_client_structs::structs::message::query::{MessageSearchQuery, MessageSearchResult};

pub struct GuildRest<'a> {
    pub guild_id: Option<u64>,
    pub client: &'a RestClient,
}

impl<'a> GuildRest<'a> {
    pub fn channel(&self, id: Option<u64>) -> Option<ChannelRest> {
        if self.guild_id.is_none() {
            return None;
        }
        Some(ChannelRest {
            channel_id: id,
            guild_id: self.guild_id.unwrap(),
            client: self.client,
        })
    }

    pub async fn create(&self, create_guild: CreateGuild) -> BoxedResult<Guild> {
        if let Some(icon) = &create_guild.icon {
            if icon.as_bytes().len() > MAX_ICON_SIZE {
                return Err("Encoded icon file is too large".into());
            }
        }

        let path = "guilds";

        let referer = HomePageReferer {};

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .post::<Guild, CreateGuild>(&path, Some(create_guild), Some(props))
            .await
    }

    pub async fn delete(&self) -> BoxedResult<()> {
        if self.guild_id.is_none() {
            return Err("Guild ID is required".into());
        }

        let path = format!("guilds/{}", self.guild_id.unwrap());

        let referer = GuildReferer {
            guild_id: self.guild_id.unwrap(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client.delete(&path, None::<&()>, Some(props)).await
    }

    pub async fn search_guild_messages(
        &self,
        query: MessageSearchQuery,
    ) -> BoxedResult<MessageSearchResult> {
        if self.guild_id.is_none() {
            return Err("Guild ID is required".into());
        }

        let path = format!("guilds/{}/messages/search", self.guild_id.unwrap());

        let referer = GuildReferer {
            guild_id: self.guild_id.unwrap(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<MessageSearchResult>(&path, Some(query.to_map()), Some(props))
            .await
    }

    pub async fn get_audit_log(&self, query: AuditLogQuery) -> BoxedResult<AuditLog> {
        if self.guild_id.is_none() {
            return Err("Guild ID is required".into());
        }

        let path = format!("guilds/{}/audit-logs", self.guild_id.unwrap());

        let referer = GuildReferer {
            guild_id: self.guild_id.unwrap(),
        };

        let props = RequestPropertiesBuilder::default()
            .referer::<Referer>(referer.into())
            .build()?;

        self.client
            .get::<AuditLog>(&path, Some(query.to_map()), Some(props))
            .await
    }
}
