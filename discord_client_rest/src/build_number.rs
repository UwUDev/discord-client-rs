use crate::BoxedResult;
use regex::Regex;
use rquest::Impersonate::Chrome133;
use rquest::ImpersonateOS::Windows;
use rquest::{Client, Impersonate};

pub async fn fetch_build_number() -> BoxedResult<u32> {
    let imp = Impersonate::builder()
        .impersonate_os(Windows)
        .impersonate(Chrome133)
        .build();

    let client = Client::builder()
        .impersonate(imp)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .build()?;

    let response = client.get("https://discord.com/login").send().await?;
    let body = response.text().await?;

    let re = Regex::new(r#"/assets/sentry\.[a-f0-9]+\.js"#).unwrap();
    let js_path = re.find(&body).unwrap().as_str();

    let js_url = format!("https://discord.com{}", js_path);
    let response = client.get(&js_url).send().await?;
    let body = response.text().await?;

    let re = Regex::new(r#""buildNumber","(\d+)""#).unwrap();
    let build_number = re
        .captures(&body)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .parse()
        .unwrap();
    Ok(build_number)
}
