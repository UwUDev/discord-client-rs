use discord_client_rest::rest::RestClient;
use log::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter(None, LevelFilter::Off)
        .filter_module("reverse", LevelFilter::Debug)
        .filter_module("discord_client_rest", LevelFilter::Debug)
        .init();

    let token = std::fs::read_to_string("token.txt").unwrap();
    let client = RestClient::connect(token, None, None, None, None)
        .await
        .unwrap();

    info!("API Version: {}", client.api_version);
    info!("Build Number: {:?}", client.build_numbers);

    info!("Cookies :");
    match client.cookie_jar() {
        Some(jar) => {
            for cookie in jar.get_all() {
                info!("  {}: {}", cookie.name(), cookie.value());
            }
        }
        None => warn!("No cookies found"),
    }
}
