use crate::BoxedResult;
use crate::clearance::{get_clearance_cookie, get_invisible};
use log::warn;
use regex::Regex;
use std::time::Duration;
use wreq::{Client, Proxy, redirect};
use wreq_util::{Emulation, EmulationOS, EmulationOption};

pub(crate) const DEFAULT_API_VERSION: u8 = 9;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);
const REQUEST_TIMEOUT: Duration = Duration::from_secs(20);

pub(crate) struct Bootstrap {
    pub client: Client,
    pub api_version: u8,
}

pub(crate) fn build_emulated_client(proxy: Option<&str>) -> BoxedResult<Client> {
    let emu = EmulationOption::builder()
        .emulation(Emulation::Chrome136)
        .emulation_os(EmulationOS::Windows)
        .build();

    let mut builder = Client::builder()
        .emulation(emu)
        .gzip(true)
        .deflate(true)
        .brotli(true)
        .zstd(true)
        .cookie_store(true)
        .redirect(redirect::Policy::default())
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT);

    if let Some(proxy) = proxy {
        builder = builder.proxy(Proxy::all(proxy)?);
    }

    Ok(builder.build()?)
}

async fn fetch_app_shell(client: &Client) -> BoxedResult<String> {
    let resp = client
        .get("https://discord.com/channels/@me")
        .send()
        .await?;

    Ok(resp.text().await?)
}

fn parse_api_version(body: &str) -> BoxedResult<u8> {
    let re = Regex::new(r#""API_VERSION":(\d+)"#).unwrap();

    if let Some(caps) = re.captures(body) {
        Ok(caps
            .get(1)
            .ok_or("Failed to find API version")?
            .as_str()
            .parse::<u8>()?)
    } else {
        Err(Box::from("Failed to find API version"))
    }
}

fn parse_challenge_token(body: &str) -> Option<String> {
    Regex::new(r#"r:'([a-f0-9]+)'"#)
        .unwrap()
        .captures(body)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
}

async fn solve_clearance_from_body(client: &Client, body: &str) {
    match (get_invisible(client).await, parse_challenge_token(body)) {
        (Ok(invisible), Some(r)) => {
            if let Err(e) = get_clearance_cookie(client, invisible, r).await {
                warn!("cloudflare clearance failed (continuing without it): {}", e);
            }
        }
        (Ok(_), None) => warn!("cloudflare clearance skipped: challenge token r not found"),
        (Err(e), _) => {
            warn!(
                "cloudflare clearance setup failed (continuing without it): {}",
                e
            )
        }
    }
}

pub(crate) async fn solve_cloudflare_clearance(client: &Client) -> BoxedResult<()> {
    let body = fetch_app_shell(client).await?;
    solve_clearance_from_body(client, &body).await;
    Ok(())
}

pub(crate) async fn bootstrap_client(
    custom_api_version: Option<u8>,
    proxy: Option<&str>,
) -> BoxedResult<Bootstrap> {
    let client = build_emulated_client(proxy)?;
    let body = fetch_app_shell(&client).await?;

    let api_version = match custom_api_version {
        Some(v) => v,
        None => parse_api_version(&body)?,
    };

    solve_clearance_from_body(&client, &body).await;

    Ok(Bootstrap {
        client,
        api_version,
    })
}
