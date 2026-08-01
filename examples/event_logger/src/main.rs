use std::collections::BTreeMap;

use discord_client_gateway::events::Event;
use discord_client_gateway::events::structs::ParseErrorEvent;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::client::ClientSession;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt")
        .expect("put a user token in token.txt")
        .trim()
        .to_string();

    let client_session = ClientSession::new();
    let _rest = RestClient::connect(token.clone(), None, None, Some(client_session), None)
        .await
        .unwrap();

    let mut gateway = GatewayClient::connect(token, true, 53607934, None)
        .await
        .unwrap();

    // Count each event type we see so a final Ctrl-C summary is easy to eyeball.
    let mut seen: BTreeMap<String, u32> = BTreeMap::new();

    loop {
        let event = match gateway.next_event().await {
            Ok(event) => event,
            Err(err) => {
                eprintln!("gateway error: {err}");
                continue;
            }
        };

        *seen.entry(event.event_name().to_string()).or_default() += 1;

        match &event {
            Event::Ready(ready) => {
                let ids: Vec<u64> = ready.guilds.iter().map(|g| g.id).collect();
                println!("READY — subscribing to {} guilds", ids.len());
                if let Err(err) = gateway.bulk_guild_subscribe(ids).await {
                    eprintln!("guild subscribe failed: {err}");
                }
            }
            Event::ParseError(e) => match e.dump_to("failed_events") {
                Ok(path) => println!(
                    "PARSE-FAIL {} :: {} at '{}' -> {}",
                    e.event_type,
                    e.error,
                    e.path,
                    path.display()
                ),
                Err(io) => eprintln!("PARSE-FAIL {} (dump failed: {io})", e.event_type),
            },
            Event::Unknown(u) => {
                // Reuse ParseErrorEvent purely as a raw-dumping vehicle.
                let capture = ParseErrorEvent {
                    event_type: u.r#type.clone(),
                    op: u.op,
                    error: "unregistered event".to_string(),
                    path: String::new(),
                    raw: u.data.clone(),
                };
                match capture.dump_to("unknown_events") {
                    Ok(path) => println!("UNKNOWN {} -> {}", u.r#type, path.display()),
                    Err(io) => eprintln!("UNKNOWN {} (dump failed: {io})", u.r#type),
                }
            }
            other => println!("{}", other.event_name()),
        }
    }
}
