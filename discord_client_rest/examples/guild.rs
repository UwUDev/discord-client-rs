use discord_client_rest::image::ImageType;
use discord_client_rest::image::base64::encode_image;
use discord_client_rest::mfa::{MfaRequiredError, MfaType};
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

    let guild = client.guild(None).create(create_guild).await.unwrap();

    let logs = client
        .guild(Some(guild.id))
        .get_audit_log(Default::default())
        .await
        .unwrap();

    println!("Got {} logs", logs.audit_log_entries.len());

    let result = client.guild(Some(guild.id)).delete().await; // boxed result

    if let Err(err) = &result {
        if let Some(mfa_error) = err.downcast_ref::<MfaRequiredError>() {
            let verification_request = &mfa_error.verification_request;
            let ticket = verification_request.clone().ticket;
            let mut found_totp_method = false;
            for method in &verification_request.methods {
                if let MfaType::Totp() = method.r#type {
                    found_totp_method = true;

                    println!("Enter your TOTP code:");
                    let mut code = String::new();
                    std::io::stdin().read_line(&mut code).unwrap();

                    client
                        .auth()
                        .validate_mfa(code.trim().to_string(), MfaType::Totp(), ticket)
                        .await
                        .unwrap();

                    client.guild(Some(guild.id)).delete().await.unwrap();

                    break;
                }
            }

            if !found_totp_method {
                println!("No TOTP method found");
                result.unwrap();
            }
        } else {
            result.unwrap();
        }
    }
}
