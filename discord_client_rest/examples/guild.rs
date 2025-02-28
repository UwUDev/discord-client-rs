use discord_client_rest::rest::RestClient;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let guild_id: u64 = 1154763102554951811;

    let logs = client
        .guild(guild_id)
        .get_audit_log(Default::default())
        .await
        .unwrap();

    println!("Got {} logs", logs.audit_log_entries.len());
}
