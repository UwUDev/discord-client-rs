use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use std::process::exit;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let mut client = GatewayClient::connect(token, true, 53607934, 369195)
        .await
        .unwrap();

    for _ in 0..2 {
        let event = client.next_event().await.unwrap();
        if let Event::Ready(ready) = event {
            let mut ids: Vec<u64> = Vec::new();
            let guilds = ready.guilds;
            for guild in guilds {
                let guild_id = guild.id;
                ids.push(guild_id);
            }

            client.bulk_guild_subscribe(ids).await.unwrap();
        }
    }
    println!("Session ID: {:?}", client.session_id.clone());
    println!("Analytics Token: {:?}", client.analytics_token.clone());
    println!(
        "Auth Session ID Hash: {:?}",
        client.auth_session_id_hash.clone()
    );

    let guild_id: u64 = 442252698964721669;
    let channel_ids: Vec<u64> = vec![443150878111694848, 448238009733742612];

    client.request_channel_statuses(guild_id).await.unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        if let Event::ChannelStatues(statues_event) = event.clone() {
            println!(
                "Got {} channel statuses for guild {}",
                statues_event.channels.len(),
                statues_event.guild_id
            );
            break;
        }
    }

    client
        .request_last_messages(guild_id, channel_ids)
        .await
        .unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        if let Event::LastMessages(last_messages) = event.clone() {
            println!(
                "Got {} last messages for guild {}",
                last_messages.messages.len(),
                last_messages.guild_id
            );
            break;
        }
    }

    // Empty query to get all members (max 1k per page, you can use pagination to get more but can't go over 10k)
    client
        .search_recent_members(guild_id, "", None, None)
        .await
        .unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        if let Event::GuildMembersChunk(members_chunk) = event.clone() {
            println!(
                "Got {} members for guild {}",
                members_chunk.members.len(),
                members_chunk.guild_id
            );
            break;
        }
    }

    // Can trigger multiple guild members chunk events
    // You need at least one oh these 3 permissions: MANAGE_ROLES, KICK_MEMBERS, BAN_MEMBERS
    client
        .request_guild_members(752493878334193674, None, None, None, None, None)
        .await
        .unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        if let Event::GuildMembersChunk(members_chunk) = event.clone() {
            println!(
                "Got {} members for guild {}",
                members_chunk.members.len(),
                members_chunk.guild_id
            );
            if members_chunk.chunk_index + 1 == members_chunk.chunk_count {
                break;
            }
        }
    }

    client
        .request_soundboard_sounds(vec![guild_id])
        .await
        .unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        println!("{}", event);
        if let Event::SoundboardSounds(sounds) = event.clone() {
            println!(
                "Got {} soundboard sounds for guild {}",
                sounds.soundboard_sounds.len(),
                sounds.guild_id
            );
            break;
        } else if let Event::Unknown(unknown) = event {
            println!("Unknown event found: {}  ({})", unknown.r#type, unknown.op);
            println!("See the event.json file to debug this event.");
            exit(0);
        }
    }

    client.graceful_shutdown().await.unwrap();
}
