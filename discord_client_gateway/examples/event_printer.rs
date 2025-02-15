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

    loop {
        let event = client.next_event().await.unwrap();
        println!("{}", event);
        if let Event::Unknown(unknown) = event {
            if unknown.event_type == "VOICE_CHANNEL_STATUS_UPDATE" {
                exit(0);
            }
        }
    }
}
