use discord_client_rest::rest::RestClient;
use discord_client_rest::structs::referer::{HomePageReferer, Referer};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let user_id: u64 = 1051578679593009212;

    let channel = client
        .dm()
        .open_or_create_dm_channel(user_id, Referer::HomePage(HomePageReferer), None)
        .await
        .unwrap();

    println!("Channel ID: {}", channel.id);

    let count = client.dm().get_dm_channels().await.unwrap().len();

    println!("DM Channel Count: {}", count);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    client.dm().close_dm_channel(channel.id).await.unwrap();
}
