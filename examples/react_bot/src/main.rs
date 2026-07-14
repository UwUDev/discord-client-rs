use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::client::ClientSession;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let client_session = ClientSession::new();

    let rest_client = RestClient::connect(token.clone(), None, None, Some(client_session))
        .await
        .unwrap();

    let mut gateway_client = GatewayClient::connect(token, true, 53607934, None)
        .await
        .unwrap();

    //let user_id = rest_client.user_id;

    for _ in 0..2 {
        let event = gateway_client.next_event().await.unwrap();

        if let Event::Ready(ready) = event {
            let mut ids: Vec<u64> = Vec::new();
            let guilds = ready.guilds;
            for guild in guilds {
                let guild_id = guild.id;
                ids.push(guild_id);
            }

            let count = ids.len();

            gateway_client.bulk_guild_subscribe(ids).await.unwrap();

            println!("Subscribed to {} guilds", count);
        }
    }

    loop {
        let event = match gateway_client.next_event().await {
            Ok(event) => event,
            Err(_) => continue,
        };

        if let Event::MessageCreate(message_create) = event {
            if message_create.message.author.id != 464412791437131776 {
                continue;
            }

            match rest_client
                .message(message_create.message.channel_id)
                .add_reaction(
                    message_create.message.id,
                    "pregnant:1483054477031506011".parse().unwrap(),
                    false,
                    message_create.message.guild_id,
                )
                .await
            {
                Ok(_) => {
                    let url = match message_create.message.guild_id {
                        None => format!(
                            "https://discord.com/channels/{}/{}",
                            message_create.message.channel_id, message_create.message.id
                        ),
                        Some(id) => format!(
                            "https://discord.com/channels/{id}/{}/{}",
                            message_create.message.channel_id, message_create.message.id
                        ),
                    };
                    println!("Reacted: {url}");
                }
                Err(err) => eprintln!("{}", err),
            }
        }
    }
}
