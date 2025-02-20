#[derive(Debug, Clone)]
pub struct GuildChannelReferer {
    pub guild_id: u64,
    pub channel_id: u64,
}

#[derive(Debug, Clone)]
pub struct DmChannelReferer {
    pub channel_id: u64,
}

#[derive(Debug, Clone)]
pub struct GuildReferer {
    pub guild_id: u64,
}

impl From<GuildChannelReferer> for Referer {
    fn from(referer: GuildChannelReferer) -> Self {
        Referer::GuildChannel(referer)
    }
}

impl From<DmChannelReferer> for Referer {
    fn from(referer: DmChannelReferer) -> Self {
        Referer::DmChannel(referer)
    }
}

impl From<GuildReferer> for Referer {
    fn from(referer: GuildReferer) -> Self {
        Referer::Guild(referer)
    }
}

#[derive(Debug, Clone)]
pub enum Referer {
    GuildChannel(GuildChannelReferer),
    DmChannel(DmChannelReferer),
    Guild(GuildReferer),
}
