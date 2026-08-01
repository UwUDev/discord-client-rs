use discord_client_structs::structs::client::BuildNumbers;
use std::error::Error;
use wreq::{Client, redirect};
use wreq_util::{Emulation, Platform, Profile};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

pub async fn find_build_numbers() -> BoxedResult<BuildNumbers> {
    let emu = Emulation::builder()
        .profile(Profile::Chrome149)
        .platform(Platform::Windows)
        .build();

    let client = Client::builder()
        .emulation(emu)
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
