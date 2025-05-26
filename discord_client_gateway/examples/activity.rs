use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_structs::structs::user::activity::{
    ActivityAssetBuilder, ActivityBuilder, ActivityTimestampBuilder,
};
use discord_client_structs::structs::user::status::StatusType::DoNotDisturb;
use std::io::Write;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let mut client = GatewayClient::connect(token, true, 53607934, 402402)
        .await
        .unwrap();

    let assets = ActivityAssetBuilder::default()
        .large_image("1353757548716753056")
        .build()
        .unwrap();

    let timestamps = ActivityTimestampBuilder::default()
        .start(chrono::Utc::now())
        .end(chrono::Utc::now() + chrono::Duration::days(30))
        .build()
        .unwrap();

    let activity = ActivityBuilder::default()
        .application_id(1147479351898669106u64)
        .name("discord-client-rs")
        .details("A Discord client written in Rust")
        .state("This library is still in development")
        .r#type(2) // music
        .assets(assets)
        .timestamps(timestamps)
        .build()
        .unwrap();

    client.activities.push(activity);
    client.status = DoNotDisturb;

    client.update_presence().await.unwrap();

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

    let mut count = 100;
    loop {
        let _ = client.next_event().await.unwrap();
        print!("\rStopping in {} events...", count);
        std::io::stdout().flush().unwrap();
        count -= 1;
        if count == 0 {
            break;
        }
    }

    client.activities = Vec::new();
    client.update_presence().await.unwrap();
    client.graceful_shutdown().await.unwrap();
}
