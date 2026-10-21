use discord_client_structs::structs::client::BuildNumbers;
use std::error::Error;
use wreq::{Client, ClientBuilder, redirect};
use wreq_util::{Emulation, Platform, Profile};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

pub const CHROME_MAJOR: &str = "154";
pub const CHROME_VERSION: &str = "154.0.0.0";
pub const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";
pub const APP_VERSION: &str =
    "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";

pub fn chrome_client_builder() -> ClientBuilder {
    let emu = Emulation::builder()
        .profile(Profile::Chrome154)
        .platform(Platform::Windows)
        .build();

    Client::builder().emulation(emu)
}

pub async fn find_build_numbers() -> BoxedResult<BuildNumbers> {
    let client = chrome_client_builder()
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .redirect(redirect::Policy::default())
        .build()?;

    let res = client
        .get("https://discord.com/app")
        .send()
        .await?
        .text()
        .await?;

    let client_build_number = res
        .split("\"BUILD_NUMBER\":\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .ok_or("Failed to find BUILD_NUMBER")?
        .to_string();

    let client_build_number: u32 = client_build_number.parse()?;

    Ok(BuildNumbers {
        client_build_number,
        native_build_number: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_build_nums() {
        let build_nums = find_build_numbers().await.unwrap();
        println!("{:?}", build_nums);
    }
}
