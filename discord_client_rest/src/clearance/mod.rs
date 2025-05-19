mod compressor;

use crate::BoxedResult;
use rquest::Client;

const LONG_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36 OPR/118.0.0.0";
const SHORT_USER_AGENT: &str = "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36 OPR/118.0.0.0";
const OPERATING_SYSTEM: &str = "Windows NT 10.0; Win64; x64";

pub(crate) async fn get_invisible(client: &Client) -> BoxedResult<(String, String)> {
    let response = client
        .get("https://discord.com/cdn-cgi/challenge-platform/scripts/jsd/main.js")
        .send()
        .await?;
    let text = response.text().await?;
    let re = regex::Regex::new(r"0\.\d+:\d+:[a-zA-Z0-9-_]+").unwrap();
    let s = re
        .captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();
    let js = text.split("'.split(','),").collect::<Vec<&str>>()[0]
        .split("='")
        .last()
        .unwrap();

    let mut key = String::new();

    for elem in js.split(",") {
        if elem.len() == 65 {
            key = elem.to_string();
        }
    }

    Ok((key, s))
}

pub(crate) async fn get_clearance_cookie(
    client: &Client,
    r: String,
    key: String,
    s: String,
) -> BoxedResult<()> {
    let url = format!("https://discord.com/cdn-cgi/challenge-platform/h/g/jsd/r/{s}/{r}");

    let current_time = chrono::Utc::now();
    let date = current_time.format("%m/%d/%Y %H:%M:%S").to_string();

    let data = include_str!("payload.json");
    let data = data
        .replace("LONG_USER_AGENT", LONG_USER_AGENT)
        .replace("SHORT_USER_AGENT", SHORT_USER_AGENT)
        .replace("OPERATING_SYSTEM", OPERATING_SYSTEM)
        .replace("CURRENT_DATE", &date);

    let compressor = compressor::Compressor::new(key.as_bytes().to_vec());
    let payload = compressor.compress(data.as_str());

    let response = client
        .post(&url)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(payload)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get clearance cookie: {}", response.status()).into());
    }

    // check if set_cookie with cf_clearance is present
    let cookies = response.cookies();
    for cookie in cookies {
        if cookie.name() == "cf_clearance" {
            return Ok(());
        }
    }

    Err("Failed to get cf_clearance cookie, no cookie found".into())
}
