use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use std::process::exit;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let mut client = GatewayClient::connect(token, 53607934, 369195)
        .await
        .unwrap();

    for _ in 0..2 {
        let event = client.next_event().await.unwrap();
        println!("{}", event);

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

    client.request_channel_statuses(guild_id).await.unwrap();

    loop {
        let event = client.next_event().await.unwrap();
        println!("{}", event);
        if let Event::ChannelStatues(statues_event) = event.clone() {
            println!(
                "Got {} channel statuses for guild {}",
                statues_event.channels.len(),
                statues_event.guild_id
            );
        } else if let Event::Unknown(unknown) = event {
            println!("Unknown event found: {}  ({})", unknown.r#type, unknown.op);
            println!("See the event.json file to debug this event.");
            exit(0);
        }
    }
}
