use base64::Engine;
use discord_client_structs::structs::channel::invite::Invite;
use serde_json::json;

pub trait ContextHeader {
    fn get_header_value(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct NoContext;

#[derive(Debug, Clone)]
pub struct NewGroupDmContext;

#[derive(Debug, Clone)]
pub struct InviteContext {
    pub location_guild_id: u64,
    pub location_channel_id: u64,
    pub location_channel_type: u8,
}

impl ContextHeader for NoContext {
    fn get_header_value(&self) -> String {
        json!({}).to_string()
    }
}

impl ContextHeader for NewGroupDmContext {
    fn get_header_value(&self) -> String {
        json!({"location":"New Group DM"}).to_string()
    }
}

impl ContextHeader for InviteContext {
    fn get_header_value(&self) -> String {
        json!({
            "location": "Join Guild",
            "location_guild_id": self.location_guild_id.to_string(),
            "location_channel_id": self.location_channel_id.to_string(),
            "location_channel_type": self.location_channel_type,
        })
        .to_string()
    }
}

#[derive(Debug, Clone)]
pub enum Context {
    NoContext,
    NewGroupDmContext,
    InviteContext(InviteContext),
}

impl ContextHeader for Context {
    fn get_header_value(&self) -> String {
        let ctx = match self {
            Context::NoContext => NoContext.get_header_value(),
            Context::NewGroupDmContext => NewGroupDmContext.get_header_value(),
            Context::InviteContext(ctx) => ctx.get_header_value(),
        };

        base64::engine::general_purpose::STANDARD.encode(ctx.as_bytes())
    }
}

impl From<Invite> for Context {
    fn from(invite: Invite) -> Self {
        match invite.guild {
            Some(guild) => {
                let channel = invite.channel.unwrap();
                let id = channel.id;
                let r#type = channel.r#type;
                Context::InviteContext(InviteContext {
                    location_guild_id: guild.id,
                    location_channel_id: id,
                    location_channel_type: r#type,
                })
            }
            None => Context::NoContext,
        }
    }
}
