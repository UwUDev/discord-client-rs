use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::message::MessageBuilder;
use discord_client_structs::structs::message::query::MessageSearchQueryBuilder;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let message = MessageBuilder::default()
        .content("Hello, World!")
        .tts(false)
        .build()
        .unwrap();

    let channel_id: u64 = 1264989590926921769;
    let guild_id: u64 = 1154763102554951811;

    let messages = client
        .message(channel_id)
        .get_channel_messages(Some(guild_id), Default::default())
        .await
        .unwrap();

    println!("Got {} messages", messages.len());

    let search_query = MessageSearchQueryBuilder::default()
        .author_id(vec![client.user_id])
        .build()
        .unwrap();

    let search_result = client
        .guild(Some(guild_id))
        .search_guild_messages(search_query)
        .await
        .unwrap();

    println!("Got {} messages from you", search_result.messages.len());

    let mut response_message = client
        .message(channel_id)
        .send(Some(guild_id), message)
        .await
        .unwrap();
    println!("Response content: {}", response_message.content.unwrap());
    let id = response_message.id;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    response_message.content = Some("Hello, World! (edited)".to_string());

    response_message = client
        .message(channel_id)
        .edit(id, Some(guild_id), response_message)
        .await
        .unwrap();

    client
        .message(channel_id)
        .add_reaction(
            response_message.id,
            String::from("🏳️‍⚧️"),
            false,
            Some(guild_id),
        )
        .await
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client
        .message(channel_id)
        .remove_reaction(
            response_message.id,
            String::from("🏳️‍⚧️"),
            None,
            false,
            Some(guild_id),
        )
        .await
        .unwrap();

    client
        .message(channel_id)
        .pin(response_message.id, Some(guild_id))
        .await
        .unwrap();

    let messages = client
        .message(channel_id)
        .get_pinned_messages(Some(guild_id))
        .await
        .unwrap();

    println!("Got {} pinned messages", messages.len());

    client
        .message(channel_id)
        .unpin(response_message.id, Some(guild_id))
        .await
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client
        .message(channel_id)
        .delete(response_message.id, Some(guild_id))
        .await
        .unwrap();
}
