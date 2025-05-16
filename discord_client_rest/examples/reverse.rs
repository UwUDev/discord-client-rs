use discord_client_rest::rest::RestClient;
use log::*;
use rquest::Url;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Off)
        .filter_module("reverse", LevelFilter::Debug)
        .filter_module("discord_client_rest", LevelFilter::Debug)
        .init();

    let token = std::fs::read_to_string("token.txt").unwrap();
    let client = RestClient::connect(token, None, None).await.unwrap();

    info!("API Version: {}", client.api_version);
    info!("Build Number: {}", client.build_number);
    info!("Cookies :");

    let cookies = client
        .get_http_client()
        .get_cookies(&Url::parse("https://discord.com").unwrap());
    if let Some(cookie_header) = cookies {
        for cookie in cookie_header.to_str().unwrap().split("; ") {
            let key_value: Vec<&str> = cookie.split('=').collect();
            if key_value.len() == 2 {
                let key = key_value[0];
                let value = key_value[1];
                info!("  {}: {}", key, value);
            } else {
                error!("Invalid cookie format: {}", cookie);
            }
        }
    } else {
        warn!("No cookies found");
    }
}
