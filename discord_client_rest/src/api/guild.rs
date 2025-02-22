use crate::api::channel::ChannelRest;
use crate::rest::RestClient;

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
}
