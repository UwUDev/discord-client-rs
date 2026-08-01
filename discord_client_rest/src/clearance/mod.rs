mod compressor;

use crate::BoxedResult;
use wreq::Client;

/// Must stay in sync with the wreq emulation used by the client (Chrome149 on Windows):
/// the challenge compares what the payload claims with what the request headers said.
const USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
const APP_VERSION: &str = "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/149.0.0.0 Safari/537.36";
const PLATFORM: &str = "Win32";
const LANGUAGE: &str = "en-US";
const LANGUAGES: &str = "en-US,en";

pub(crate) struct Invisible {
    pub key: String,
    pub s: String,
    pub zone: String,
    pub hash: String,
}

pub(crate) async fn get_invisible(client: &Client) -> BoxedResult<Invisible> {
    let response = client
        .get("https://discord.com/cdn-cgi/challenge-platform/scripts/jsd/main.js")
        .send()
        .await?;

    let final_url = response.uri().to_string();
    let path_re = regex::Regex::new(r"/h/([^/]+)/scripts/jsd/([^/]+)/main\.js")?;
    let (zone, hash) = path_re
        .captures(&final_url)
        .map(|c| (c[1].to_string(), c[2].to_string()))
        .ok_or("Failed to find zone/hash in jsd script URL")?;

    let text = response.text().await?;

    let s = regex::Regex::new(r"0\.\d+:\d+:[a-zA-Z0-9_-]+")?
        .find(&text)
        .ok_or("Failed to find s in jsd script")?
        .as_str()
        .to_string();

    let key = regex::Regex::new(r#"[`'"]([A-Za-z0-9+/=_$.\-]{64,65})[`'"]"#)?
        .captures_iter(&text)
        .map(|c| c[1].to_string())
        .find(|v| v.chars().collect::<std::collections::HashSet<_>>().len() >= 60)
        .ok_or("Failed to find the alphabet key in jsd script")?;

    Ok(Invisible { key, s, zone, hash })
}

pub(crate) async fn get_clearance_cookie(
    client: &Client,
    invisible: Invisible,
    r: String,
) -> BoxedResult<()> {
    let Invisible { key, s, zone, hash } = invisible;

    let url = format!(
        "https://discord.com/cdn-cgi/challenge-platform/h/{zone}/jsd/oneshot/{hash}/{s}/{r}"
    );

    let current_time = chrono::Utc::now();
    let date = current_time.format("%m/%d/%Y %H:%M:%S").to_string();

    let signals = include_str!("payload.json")
        .replace("USER_AGENT", USER_AGENT)
        .replace("APP_VERSION", APP_VERSION)
        .replace("PLATFORM", PLATFORM)
        .replace("LANGUAGES", LANGUAGES)
        .replace("LANGUAGE", LANGUAGE)
        .replace("CURRENT_DATE", &date);

    let data = format!(
        r#"{{"t":{},"lhr":"about:blank","api":false,"c":false,"payload":{signals}}}"#,
        current_time.timestamp()
    );

    let compressor = compressor::Compressor::new(key.as_bytes().to_vec());
    let payload = compressor.compress(&data);

    let response = client
        .post(&url)
        .header("Content-Type", "text/plain;charset=UTF-8")
        .body(payload)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(format!("Failed to get clearance cookie: {}", response.status()).into());
    }

    let cookies = response.cookies();
    for cookie in cookies {
        if cookie.name() == "cf_clearance" {
            return Ok(());
        }
    }

    Err("Failed to get cf_clearance cookie, no cookie found".into())
}
