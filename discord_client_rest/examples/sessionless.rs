use discord_client_rest::sessionless::SessionlessClient;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let client = SessionlessClient::connect(None, None, None, None, None, true)
        .await
        .unwrap();

    println!("API Version: {}", client.api_version);

    let assignments = client.experiments().get_assignments(true).await.unwrap();

    println!("Fingerprint: {:?}", client.fingerprint().await);
    println!("Assignments: {}", assignments.assignments.len());

    let invite = client
        .invite()
        .get_invite("discord-developers".to_string())
        .await
        .unwrap();

    println!("Invite guild: {:?}", invite.guild.map(|g| g.name));
}
