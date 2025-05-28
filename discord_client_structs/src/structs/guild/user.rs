use crate::structs::channel::overrides::ChannelOverride;
use crate::structs::guild::deserialize_option_string_to_u64;
use crate::structs::misc::MuteConfig;
use discord_client_macros::Flags;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Flags)]
pub struct UserGuildSettings {
    pub channel_overrides: Vec<ChannelOverride>,
    #[flag_enum(
        "UnreadsAllMessages=11,UnreadsOnlyMentions=12,OptInChannelsOff=13,OptInChannelsOn=14"
    )]
    pub flags: u64,
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_option_string_to_u64")]
    #[serde(serialize_with = "serialize_option_u64_as_string")]
    pub guild_id: Option<u64>,
    pub hide_muted_channels: bool,
    pub message_notifications: u64,
    pub mobile_push: bool,
    pub mute_scheduled_events: bool,
    pub muted: bool,
    pub mute_config: Option<MuteConfig>,
    pub notify_highlights: u64,
    pub suppress_everyone: bool,
    pub suppress_roles: bool,
    pub version: u64,
}

#[cfg(test)]
pub mod test_macro {
    use super::*;

    #[test]
    fn test_user_guild_settings_flags() {
        let json = r#"{
            "channel_overrides": [],
            "flags": 6144,
            "guild_id": "123456789012345678",
            "hide_muted_channels": true,
            "message_notifications": 1,
            "mobile_push": false,
            "mute_scheduled_events": true,
            "muted": false,
            "mute_config": null,
            "notify_highlights": 0,
            "suppress_everyone": false,
            "suppress_roles": true,
            "version": 1
        }"#;

        let settings: UserGuildSettings = serde_json::from_str(json).unwrap();
        println!("Encoded flags: {}", settings.flags);

        let active_flags = settings.get_flags();
        println!("Base flags: {:?}", active_flags);

        if settings.has_flags(UserGuildSettingsFlags::UnreadsAllMessages) {
            println!("UnreadsAllMessages added");
        }

        if settings.has_flags(UserGuildSettingsFlags::OptInChannelsOn) {
            println!("OptInChannelsOn is activated");
        } else {
            println!("OptInChannelsOn is not activated");
        }

        let mut settings_mut = settings.clone();
        settings_mut.add_flags(UserGuildSettingsFlags::OptInChannelsOn);
        println!(
            "After adding OptInChannelsOn: {:?}",
            settings_mut.get_flags()
        );

        settings_mut.remove_flags(UserGuildSettingsFlags::UnreadsAllMessages);
        println!("After removal: {:?}", settings_mut.get_flags());

        settings_mut.set_flags(vec![
            UserGuildSettingsFlags::OptInChannelsOff,
            UserGuildSettingsFlags::UnreadsOnlyMentions,
        ]);
        println!("After setting bulk flags: {:?}", settings_mut.get_flags());
        println!("Final encoded flags: {}", settings_mut.flags);
    }
}
