use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::channel::ChannelBuilder;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let guild_id: u64 = 1154763102554951811;

    let channel = ChannelBuilder::default()
        .name("test-channel")
        .build()
        .unwrap();

    let mut channel = client
        .guild(Some(guild_id))
        .channel(None)
        .unwrap()
        .create(channel)
        .await
        .unwrap();

    let channel_id = channel.id;
    println!("Channel ID: {}", channel_id);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    channel.name = Some("test-channel-edited".to_string());

    client
        .guild(Some(guild_id))
        .channel(None)
        .unwrap()
        .edit(channel)
        .await
        .unwrap();
    let invite = client
        .guild(Some(guild_id))
        .channel(Some(channel_id))
        .unwrap()
        .create_invite(Default::default())
        .await
        .unwrap();

    println!("Invite: discord.gg/{}", invite.code);

    let invite_count = client
        .guild(Some(guild_id))
        .channel(Some(channel_id))
        .unwrap()
        .get_invites()
        .await
        .unwrap()
        .len();

    println!("Invite count: {}", invite_count);

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    client
        .guild(Some(guild_id))
        .channel(Some(channel_id))
        .unwrap()
        .delete()
        .await
        .unwrap();
}
