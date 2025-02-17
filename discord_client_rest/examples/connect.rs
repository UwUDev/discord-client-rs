use discord_client_rest::rest::RestClient;
use serde_json::Value;

#[derive(serde::Serialize)]
struct MessagePayload {
    content: String,
    tts: bool,
}

#[tokio::main]
async fn main() {
    let build_num = 369195;

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, build_num).await.unwrap();

    println!("API Version: {}", client.api_version);

    let message = MessagePayload {
        content: "Hello, world!".to_string(),
        tts: false,
    };

    let path = "channels/1264989590926921769/messages";

    let response = client.post::<Value, _>(path, Some(&message)).await.unwrap();

    println!("Response: {}", response.to_string());
    let id = response["id"].as_str().unwrap();
    let path = format!("{}/{}", path, id);

    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    client
        .delete::<Value, _>(path.as_str(), None::<&()>)
        .await
        .unwrap();
}
