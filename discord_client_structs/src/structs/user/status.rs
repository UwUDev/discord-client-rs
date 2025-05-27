use discord_client_macros::EnumFromString;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFromString)]
#[str_value("online")]
pub enum StatusType {
    Online,
    #[str_value("dnd")]
    DoNotDisturb,
    #[str_value("idle")]
    AFK,
    Invisible,
    Offline,
    Unknown,
}
