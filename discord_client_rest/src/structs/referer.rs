pub trait RefererHeader {
    fn get_header_value(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct HomePageReferer;

impl RefererHeader for HomePageReferer {
    fn get_header_value(&self) -> String {
        String::from("https://discord.com/channels/@me")
    }
}

#[derive(Debug, Clone)]
pub struct GuildChannelReferer {
    pub guild_id: u64,
    pub channel_id: u64,
}

impl RefererHeader for GuildChannelReferer {
    fn get_header_value(&self) -> String {
        format!(
            "https://discord.com/channels/{}/{}",
            self.guild_id, self.channel_id
        )
    }
}

#[derive(Debug, Clone)]
pub struct DmChannelReferer {
    pub channel_id: u64,
}

impl RefererHeader for DmChannelReferer {
    fn get_header_value(&self) -> String {
        format!("https://discord.com/channels/@me/{}", self.channel_id)
    }
}

#[derive(Debug, Clone)]
pub struct GuildReferer {
    pub guild_id: u64,
}

impl RefererHeader for GuildReferer {
    fn get_header_value(&self) -> String {
        format!("https://discord.com/channels/{}", self.guild_id)
    }
}

impl From<HomePageReferer> for Referer {
    fn from(referer: HomePageReferer) -> Self {
        Referer::HomePage(referer)
    }
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
    HomePage(HomePageReferer),
    GuildChannel(GuildChannelReferer),
    DmChannel(DmChannelReferer),
    Guild(GuildReferer),
}

impl RefererHeader for Referer {
    fn get_header_value(&self) -> String {
        match self {
            Referer::HomePage(referer) => referer.get_header_value(),
            Referer::GuildChannel(referer) => referer.get_header_value(),
            Referer::DmChannel(referer) => referer.get_header_value(),
            Referer::Guild(referer) => referer.get_header_value(),
        }
    }
}
