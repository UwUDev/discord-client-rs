use discord_client_rest::image::base64::encode_image;
use discord_client_rest::image::ImageType;
use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::guild::create::CreateGuildBuilder;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let token = std::fs::read_to_string("token.txt").unwrap();

    let client = RestClient::connect(token, None, None).await.unwrap();

    println!("API Version: {}", client.api_version);

    let image_bytes = std::fs::read("icon.png").unwrap();
    let image_type = ImageType::Png;

    let create_guild = CreateGuildBuilder::default()
        .name("Test Guild")
        .icon(encode_image(image_bytes, image_type))
        .build()
        .unwrap();

    let guild = client
        .guild(None)
        .create(create_guild)
        .await
        .unwrap();

    let logs = client
        .guild(Some(guild.id))
        .get_audit_log(Default::default())
        .await
        .unwrap();

    println!("Got {} logs", logs.audit_log_entries.len());

    client
        .guild(Some(guild.id))
        .delete()
        .await
        .unwrap();
}
