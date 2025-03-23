use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::message::MessageBuilder;
use discord_client_structs::structs::message::MessageReferenceBuilder;

#[tokio::main]
async fn main() {
    let token = std::fs::read_to_string("token.txt").unwrap();

    let rest_client = RestClient::connect(token.clone(), None, None)
        .await
        .unwrap();

    let mut gateway_client =
        GatewayClient::connect(token, true, 53607934, rest_client.build_number)
            .await
            .unwrap();

    let user_id = rest_client.user_id;

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
        let event = gateway_client.next_event().await.unwrap();
        if let Event::MessageCreate(message_create) = event {
            if message_create.message.author.id != user_id {
                continue;
            }

            if let Some(content) = message_create.message.content {
                if content.to_lowercase().ends_with("quoi") {
                    let message_reference = MessageReferenceBuilder::default()
                        .message_id(message_create.message.id)
                        .channel_id(message_create.message.channel_id)
                        .guild_id(message_create.message.guild_id)
                        .build()
                        .unwrap();

                    let message = MessageBuilder::default()
                        .content("Feur !")
                        .message_reference(message_reference)
                        .build()
                        .unwrap();

                    let guild_id = message_create.message.guild_id;

                    rest_client
                        .message(message_create.message.channel_id)
                        .send(guild_id, message)
                        .await
                        .unwrap();
                }
            }
        }
    }
}
