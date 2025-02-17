use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::message::{Message, MessageBuilder};
use serde_json::Value;

#[tokio::main]
async fn main() {
    let build_num = 369195;

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, build_num).await.unwrap();

    println!("API Version: {}", client.api_version);

    let message = MessageBuilder::default()
        .content("Hello, World!")
        .tts(false)
        .build()
        .unwrap();

    let path = "channels/1264989590926921769/messages";

    let response_message = client
        .post::<Message, Message>(path, Some(message))
        .await
        .unwrap();

    println!("Response content: {}", response_message.content.unwrap());
    let id = response_message.id;
    let path = format!("{}/{}", path, id);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client
        .delete::<Value, _>(path.as_str(), None::<&()>)
        .await
        .unwrap();
}
