use discord_client_rest::rest::RestClient;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let channel = client
        .group()
        .create_group(vec![1258065464966254600, 901809620165722112])
        .await
        .unwrap();

    for user in &channel.recipients.unwrap() {
        client
            .group()
            .kick_user_from_group(channel.id, user.id)
            .await
            .unwrap();
    }

    client
        .group()
        .rename_group(channel.id, "New Group Name")
        .await
        .unwrap();

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    client.group().leave_group(channel.id, false).await.unwrap();
}
