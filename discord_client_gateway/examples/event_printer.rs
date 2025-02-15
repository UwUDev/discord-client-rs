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
    }
    println!("Session ID: {:?}", client.session_id.clone());
    println!("Analytics Token: {:?}", client.analytics_token.clone());
    println!(
        "Auth Session ID Hash: {:?}",
        client.auth_session_id_hash.clone()
    );

    // todo: CONTENT_INVENTORY_INBOX_STALE
    // todo: USER_SETTINGS_PROTO_UPDATE
    // todo: MESSAGE_REACTION_REMOVE
    // todo: GUILD_INTEGRATIONS_UPDATE
    // todo: INTEGRATIONS_UPDATE
    // todo: SESSIONS_REPLACE
    // todo: MESSAGE_ACK (read/unread)

    loop {
        let event = client.next_event().await.unwrap();
        println!("{}", event);
        if let Event::Unknown(unknown) = event {
            if unknown.event_type == "SESSIONS_REPLACE" {
                exit(0);
            }
        }
        /*if let Event::GatewayReconnect(_)= event {
            exit(0);
        }*/
    }
}
