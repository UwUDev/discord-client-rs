use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use std::process::exit;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let mut client = GatewayClient::connect(token).await.unwrap();
    for _ in 0..2 {
        let event = client.next_event().await.unwrap();
        println!("{}", event);

        if let Event::Ready(ready)= event {
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

    // todo: CONTENT_INVENTORY_INBOX_STALE
    // todo: USER_SETTINGS_PROTO_UPDATE
    // todo: GUILD_INTEGRATIONS_UPDATE
    // todo: INTEGRATIONS_UPDATE
    // todo: SESSIONS_REPLACE

    loop {
        let event = client.next_event().await.unwrap();
        println!("{}", event);
        if let Event::Unknown(unknown) = event {
            if unknown.event_type == "GUILD_CREATE" {
                exit(0);
            }
        }
    }
}
