use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use std::collections::HashSet;
use tokio::time::{Duration, Instant};

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let start = Instant::now();

    let mut client = GatewayClient::connect(token, 53607934, 369195)
        .await
        .unwrap();

    let mut ids: Vec<u64> = Vec::new();
    for _ in 0..2 {
        let event = client.next_event().await.unwrap();
        if let Event::Ready(ready) = event {
            let guilds = ready.guilds;
            for guild in guilds {
                let guild_id = guild.id;
                ids.push(guild_id);
            }

            client.bulk_guild_subscribe(ids.clone()).await.unwrap();
        }
    }

    let mut unique_users: HashSet<u64> = HashSet::new();

    for guild_id in ids.clone() {
        client
            .search_recent_members(guild_id, "", None, None)
            .await
            .unwrap();

        let mut count = 0;
        loop {
            let result = client.next_event().await;

            if result.is_err() {
                println!("Error: {:?}", result.err());
                println!("Reconnecting");

                tokio::time::sleep(Duration::from_secs(500)).await;

                client.reconnect().await.unwrap();
                break;
            }

            let event = result.unwrap();

            if let Event::GuildMembersChunk(members_chunk) = event.clone() {
                println!(
                    "Got {} members for guild {}",
                    members_chunk.members.len(),
                    members_chunk.guild_id
                );

                count += members_chunk.members.len();

                let old_size = unique_users.len();

                members_chunk.members.iter().for_each(|member| {
                    unique_users.insert(member.clone().user.unwrap().id);
                });

                // if no new users were added, break
                let new_size = unique_users.len();
                if new_size == old_size {
                    break;
                }

                let page_member_id = members_chunk
                    .members
                    .iter()
                    .max_by_key(|member| member.joined_at)
                    .unwrap()
                    .clone()
                    .user
                    .unwrap()
                    .id;

                // if there are no more chunks, break
                // 10000 is the max pagination limit
                // Not getting 1000 members means we are at the end of the server
                if count == 10000 || count % 1000 != 0 {
                    break;
                }

                // ask next chunk
                client
                    .search_recent_members(guild_id, "", Some(page_member_id), None)
                    .await
                    .unwrap();
            }
        }
    }

    println!("Got {} unique users", unique_users.len());

    for guild_id in ids {
        println!("Trying to request all members for guild {}", guild_id);
        client
            .request_guild_members(guild_id, None, None, None, None, None)
            .await
            .unwrap();

        let mut last_received = Instant::now();

        loop {
            tokio::select! {
                event_result = client.next_event() => {
                    if event_result.is_err() {
                        println!("Error: {:?}", event_result.err());
                        println!("Reconnecting");

                        tokio::time::sleep(Duration::from_secs(500)).await;

                        client.reconnect().await.unwrap();
                        break;
                    }

                    let event = event_result.unwrap();
                    if let Event::GuildMembersChunk(members_chunk) = event.clone() {
                        println!(
                            "Got {} members for guild {}",
                            members_chunk.members.len(),
                            members_chunk.guild_id
                        );
                        for member in members_chunk.members {
                            unique_users.insert(member.user.unwrap().id);
                        }

                        last_received = Instant::now();
                    }
                }

                // we don't get event if the user isn't allowed to request all members
                _ = tokio::time::sleep_until(last_received + Duration::from_millis(500)) => {
                    println!("No users received for 500ms, breaking");
                    break;
                }
            }
        }
    }

    let end = Instant::now();

    client.graceful_shutdown().await.unwrap();
    println!(
        "Got {} unique users in {:?}",
        unique_users.len(),
        end - start
    );
}
