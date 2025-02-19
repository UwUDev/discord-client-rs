use crate::structs::message::Message;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::Display;

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageQuery {
    pub around: Option<u64>,
    pub before: Option<u64>,
    pub after: Option<u64>,
    pub limit: u8,
}

impl MessageQuery {
    pub fn to_map(&self) -> std::collections::HashMap<String, String> {
        let mut map = std::collections::HashMap::new();
        if let Some(around) = self.around {
            map.insert("around".to_string(), around.to_string());
        }
        if let Some(before) = self.before {
            map.insert("before".to_string(), before.to_string());
        }
        if let Some(after) = self.after {
            map.insert("after".to_string(), after.to_string());
        }
        let limit = if self.limit == 0u8 || self.limit > 100u8 {
            50u8
        } else {
            self.limit
        };
        map.insert("limit".to_string(), limit.to_string());
        map
    }
}

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageSearchQuery {
    pub limit: Option<u8>,
    pub offset: Option<u16>,
    pub max_id: Option<u64>,
    pub min_id: Option<u64>,
    pub include_nsfw: Option<bool>,
    pub content: Option<String>,
    pub channel_id: Option<Vec<u64>>,
    pub author_type: Option<Vec<AuthorType>>,
    pub author_id: Option<Vec<u64>>,
    pub mentions: Option<Vec<u64>>,
    pub mention_everyone: Option<bool>,
    pub pinned: Option<bool>,
    pub has: Option<Vec<HasType>>,
    pub embed_type: Option<Vec<String>>,
    pub embed_provider: Option<Vec<String>>,
    pub link_hostname: Option<Vec<String>>,
    pub attachment_filename: Option<Vec<String>>,
    pub attachment_extension: Option<Vec<String>>,
    pub command_id: Option<Vec<u64>>,
    pub sort_by: Option<MessageSortType>,
    pub sort_order: Option<SortOrder>,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum HasType {
    Image,
    Sound,
    Video,
    File,
    Sticker,
    Embed,
    Link,
    Poll,
    Snapshot,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum AuthorType {
    User,
    Bot,
    Webhook,
}

#[derive(Debug, Clone, Display)]
#[strum(serialize_all = "snake_case")]
pub enum MessageSortType {
    Timestamp,
    Relevance,
}

impl MessageSearchQuery {
    pub fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        if let Some(limit) = self.limit {
            map.insert("limit".to_string(), limit.to_string());
        }
        if let Some(offset) = self.offset {
            map.insert("offset".to_string(), offset.to_string());
        }
        if let Some(max_id) = self.max_id {
            map.insert("max_id".to_string(), max_id.to_string());
        }
        if let Some(min_id) = self.min_id {
            map.insert("min_id".to_string(), min_id.to_string());
        }
        if let Some(include_nsfw) = self.include_nsfw {
            map.insert("include_nsfw".to_string(), include_nsfw.to_string());
        }
        if let Some(content) = &self.content {
            map.insert("content".to_string(), content.to_string());
        }
        if let Some(channel_id) = &self.channel_id {
            map.insert(
                "channel_id".to_string(),
                channel_id
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(author_type) = &self.author_type {
            map.insert(
                "author_type".to_string(),
                author_type
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(author_id) = &self.author_id {
            map.insert(
                "author_id".to_string(),
                author_id
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(mentions) = &self.mentions {
            map.insert(
                "mentions".to_string(),
                mentions
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(mention_everyone) = self.mention_everyone {
            map.insert("mention_everyone".to_string(), mention_everyone.to_string());
        }
        if let Some(pinned) = self.pinned {
            map.insert("pinned".to_string(), pinned.to_string());
        }
        if let Some(has) = &self.has {
            map.insert(
                "has".to_string(),
                has.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(embed_type) = &self.embed_type {
            map.insert(
                "embed_type".to_string(),
                embed_type
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(embed_provider) = &self.embed_provider {
            map.insert(
                "embed_provider".to_string(),
                embed_provider
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(link_hostname) = &self.link_hostname {
            map.insert(
                "link_hostname".to_string(),
                link_hostname
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(attachment_filename) = &self.attachment_filename {
            map.insert(
                "attachment_filename".to_string(),
                attachment_filename
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(attachment_extension) = &self.attachment_extension {
            map.insert(
                "attachment_extension".to_string(),
                attachment_extension
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(command_id) = &self.command_id {
            map.insert(
                "command_id".to_string(),
                command_id
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(","),
            );
        }
        if let Some(sort_by) = &self.sort_by {
            map.insert("sort_by".to_string(), sort_by.to_string());
        }
        if let Some(sort_order) = &self.sort_order {
            map.insert("sort_order".to_string(), sort_order.to_string());
        }
        map
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct MessageSearchResult {
    pub analytics_id: String,
    pub doing_deep_historical_index: bool,
    pub total_results: u16,
    pub messages: Vec<Vec<Message>>, // why ????
}
