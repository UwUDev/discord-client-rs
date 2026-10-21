use discord_client_structs::structs::client::BuildNumbers;
use std::error::Error;
use std::sync::OnceLock;
use wreq::header::{HeaderMap, HeaderValue};
use wreq::{Client, ClientBuilder, redirect};
use wreq_util::{Emulation, Platform, Profile};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

pub const CHROME_MAJOR: &str = "154";
pub const CHROME_VERSION: &str = "154.0.0.0";
pub const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";
pub const APP_VERSION: &str = "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/154.0.0.0 Safari/537.36";

// todo: automate discord client versions (i'll make another crate that download the client and parse it)
pub const DESKTOP_FALLBACK_CLIENT_VERSION: &str = "1.0.9264";
pub const ELECTRON_VERSION: &str = "42.11.15";
pub const ELECTRON_CHROME_VERSION: &str = "148.0.7778.280";
pub const DESKTOP_OS_VERSION: &str = "10.0.26100";
pub const DESKTOP_OS_SDK_VERSION: &str = "26100";
pub const DESKTOP_NATIVE_BUILD_NUMBER: u32 = 95452;
pub const DESKTOP_SEC_CH_UA: &str = "\"Not/A)Brand\";v=\"99\", \"Chromium\";v=\"148\"";
const DESKTOP_MANIFEST_URL: &str = "https://discord.com/api/updates/distributions/app/manifests/latest?channel=stable&platform=win&arch=x64";

pub fn desktop_user_agent(client_version: &str) -> String {
    format!(
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) discord/{client_version} Chrome/{ELECTRON_CHROME_VERSION} Electron/{ELECTRON_VERSION} Safari/537.36"
    )
}

pub fn chrome_client_builder() -> ClientBuilder {
    let emu = Emulation::builder()
        .profile(Profile::Chrome154)
        .platform(Platform::Windows)
        .build();

    Client::builder().emulation(emu)
}

pub fn desktop_client_builder(client_version: &str) -> ClientBuilder {
    let emu = Emulation::builder()
        .profile(Profile::Chrome148)
        .platform(Platform::Windows)
        .build();

    let mut headers = HeaderMap::new();
    headers.insert("sec-ch-ua", HeaderValue::from_static(DESKTOP_SEC_CH_UA));

    Client::builder()
        .emulation(emu)
        .default_headers(headers)
        .user_agent(desktop_user_agent(client_version))
}

pub async fn desktop_client_version() -> &'static str {
    static VERSION: OnceLock<String> = OnceLock::new();
    if let Some(version) = VERSION.get() {
        return version;
    }
    let version = fetch_desktop_client_version()
        .await
        .unwrap_or_else(|_| DESKTOP_FALLBACK_CLIENT_VERSION.to_string());
    VERSION.get_or_init(|| version)
}

async fn fetch_desktop_client_version() -> BoxedResult<String> {
    let manifest: serde_json::Value = chrome_client_builder()
        .gzip(true)
        .brotli(true)
        .build()?
        .get(DESKTOP_MANIFEST_URL)
        .send()
        .await?
        .json()
        .await?;

    let parts = manifest["full"]["host_version"]
        .as_array()
        .ok_or("Missing host_version")?;
    Ok(parts
        .iter()
        .filter_map(|p| p.as_u64())
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join("."))
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
