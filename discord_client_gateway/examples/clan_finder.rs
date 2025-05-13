use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_structs::structs::guild::clan::ClanBadge;
use discord_client_structs::structs::user::User;
use std::io::Write;
use tokio::time::Instant;

#[tokio::main]
async fn main() {
    // to get all guild infos de a PUT on https://discord.com/api/v9/guilds/{guild_id}/members/@me?lurker=true&session_id={gateway_session} with {} body
    // if the guild is not public, you gan get them partially by doing a GET on https://discord.com/api/v9/guilds/{guild_id}/profile

    let token = std::fs::read_to_string("token.txt").unwrap();

    let mut client = GatewayClient::connect(token.clone(), true, 369195, 53607934)
        .await
        .unwrap();

    let mut found: Vec<ClanBadge> = get_file_clans();
    let mut ids: Vec<u64> = Vec::new();
    for _ in 0..2 {
        let event = client.next_event().await.unwrap();
        if let Event::Ready(ready) = event.clone() {
            let guilds = ready.guilds;
            for guild in guilds {
                let guild_id = guild.id;
                ids.push(guild_id);
            }

            client.bulk_guild_subscribe(ids.clone()).await.unwrap();
        }

        if let Event::Ready(ready) = event.clone() {
            if let Some(users) = ready.users {
                for user in users {
                    verify_if_user_is_clan_member(user, &mut found);
                }
            }
            if let Some(merged_members) = ready.merged_members {
                for members in merged_members {
                    for member in members {
                        if let Some(user) = member.user {
                            verify_if_user_is_clan_member(user, &mut found);
                        }
                    }
                }
            }

            for guild in ready.guilds {
                if let Some(members) = guild.members {
                    for member in members {
                        if let Some(user) = member.user {
                            verify_if_user_is_clan_member(user, &mut found);
                        }
                    }
                }
            }
        } else if let Event::ReadySupplemental(ready_supplemental) = event.clone() {
            for members in ready_supplemental.merged_members {
                for member in members {
                    if let Some(user) = member.user {
                        verify_if_user_is_clan_member(user, &mut found);
                    }
                }
            }
        }
    }

    let mut last_request = Instant::now();

    loop {
        let event = client.next_event().await;
        if event.is_err() {
            println!("Error: {:?}", event);
            continue;
        }
        let event = event.unwrap();
        if let Event::TypingStart(typing_start_event) = event.clone() {
            if let Some(member) = typing_start_event.member {
                if let Some(user) = member.user {
                    verify_if_user_is_clan_member(user, &mut found)
                }
            }
        } else if let Event::GuildMemberUpdate(guild_member) = event.clone() {
            verify_if_user_is_clan_member(guild_member.user, &mut found)
        } else if let Event::MessageCreate(message_create) = event.clone() {
            verify_if_user_is_clan_member(message_create.message.author, &mut found)
        } else if let Event::GuildMembersChunk(guild_members_chunk) = event.clone() {
            for member in guild_members_chunk.members {
                if let Some(user) = member.user {
                    verify_if_user_is_clan_member(user, &mut found);
                }
            }
        } else if let Event::ReadySupplemental(ready_supplemental) = event.clone() {
            for members in ready_supplemental.merged_members {
                for member in members {
                    if let Some(user) = member.user {
                        verify_if_user_is_clan_member(user, &mut found);
                    }
                }
            }
        }

        if last_request.elapsed().as_secs() > 2 {
            let guild_id = ids.pop();
            if guild_id.is_none() {
                continue;
            }
            let guild_id = guild_id.unwrap();
            client
                .search_recent_members(guild_id, "", None, None)
                .await
                .unwrap();

            last_request = Instant::now();
        }
    }
}

fn verify_if_user_is_clan_member(user: User, found: &mut Vec<ClanBadge>) {
    if let Some(clan) = user.clan {
        if clan.identity_guild_id.is_none() {
            return;
        }
        if let Some(clan_id) = clan.identity_guild_id {
            if found.iter().any(|x| x.identity_guild_id == Some(clan_id)) {
                let mut found_clan: ClanBadge = found
                    .iter()
                    .find(|x| x.identity_guild_id == Some(clan_id))
                    .unwrap()
                    .clone();
                
                if let Some(id_enabled) = clan.identity_enabled {
                    found_clan.identity_enabled = Some(id_enabled);
                }
                if let Some(tag) = clan.tag {
                    found_clan.tag = Some(tag);
                }
                if let Some(badge) = clan.badge {
                    found_clan.badge = Some(badge);
                }
                
                // replace the old clan with the new one
                found.retain(|x| x.identity_guild_id != Some(clan_id));
                found.push(found_clan);
            } else {
                println!("Found clan: {:?}", clan);
                found.push(clan);
            }
            save_clans_to_file(found);
        }
    }
}

fn get_file_clans() -> Vec<ClanBadge> {
    let file = std::fs::File::open("clans.json");
    if file.is_err() {
        let content = "[]";
        let mut file = std::fs::File::create("clans.json").unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file.flush().unwrap();
        return Vec::new();
    }
    let file = file.unwrap();
    let reader = std::io::BufReader::new(file);
    let clans: Vec<ClanBadge> = serde_json::from_reader(reader).unwrap();
    clans
}

fn save_clans_to_file(ids: &Vec<ClanBadge>) {
    let open_options = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("clans.json")
        .unwrap();
    let mut file = std::io::BufWriter::new(open_options);
    let json = serde_json::to_string_pretty(ids).unwrap();
    file.write_all(json.as_bytes()).unwrap();
    file.flush().unwrap();
}
