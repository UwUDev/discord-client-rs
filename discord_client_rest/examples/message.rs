use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::message::MessageBuilder;
use discord_client_structs::structs::message::query::MessageSearchQueryBuilder;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let build_num = 369195;

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, build_num).await.unwrap();

    println!("API Version: {}", client.api_version);

    let message = MessageBuilder::default()
        .content("Hello, World!")
        .tts(false)
        .build()
        .unwrap();

    let channel_id: u64 = 1264989590926921769;
    let guild_id: u64 = 1154763102554951811;

    let messages = client
        .message()
        .get_channel_messages(channel_id, Default::default())
        .await
        .unwrap();

    println!("Got {} messages", messages.len());

    let search_query = MessageSearchQueryBuilder::default()
        .author_id(vec![client.user_id])
        .build()
        .unwrap();

    let search_result = client
        .message()
        .search_guild_messages(guild_id, search_query)
        .await
        .unwrap();

    println!("Got {} messages from you", search_result.messages.len());

    let mut response_message = client.message().send(channel_id, message).await.unwrap();
    println!("Response content: {}", response_message.content.unwrap());
    let id = response_message.id;

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    response_message.content = Some("Hello, World! (edited)".to_string());

    response_message = client
        .message()
        .edit(channel_id, id, response_message)
        .await
        .unwrap();

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client
        .message()
        .delete(channel_id, response_message.id)
        .await
        .unwrap();
}
