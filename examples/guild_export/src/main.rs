use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use serde::Serialize;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, Semaphore};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const CAPABILITIES: u32 = 53_607_934;
const MAX_CONCURRENT: usize = 8;
const INPUT_FILE: &str = "tokens.txt";
const OUTPUT_FILE: &str = "guilds.json";
const READY_TIMEOUT: Duration = Duration::from_secs(15);

#[derive(Serialize)]
struct GuildInfo {
    id: u64,
    name: Option<String>,
}

#[derive(Serialize)]
struct AccountEntry {
    token: String,
    user_id: Option<u64>,
    username: Option<String>,
    error: Option<String>,
    guilds: Vec<GuildInfo>,
}

fn mask(token: &str) -> String {
    if token.len() <= 12 {
        return "*".repeat(token.len());
    }
    format!("{}...{}", &token[..8], &token[token.len() - 4..])
}

async fn fetch_guilds(token: String, build_number: u32) -> AccountEntry {
    let label = mask(&token);

    let mut client = match GatewayClient::connect(
        token.clone(),
        false,
        CAPABILITIES,
        Some(build_number),
    )
    .await
    {
        Ok(client) => client,
        Err(e) => {
            eprintln!("[error] {label}: {e}");
            return AccountEntry {
                token,
                user_id: None,
                username: None,
                error: Some(e.to_string()),
                guilds: Vec::new(),
            };
        }
    };

    let ready = loop {
        match tokio::time::timeout(READY_TIMEOUT, client.next_event()).await {
            Ok(Ok(Event::Ready(ready))) => break Some(ready),
            Ok(Ok(_other)) => continue,
            Ok(Err(e)) => {
                eprintln!("[error] {label}: {e}");
                break None;
            }
            Err(_) => {
                eprintln!("[error] {label}: timed out waiting for READY");
                break None;
            }
        }
    };

    let _ = client.graceful_shutdown().await;

    match ready {
        Some(ready) => {
            let guilds: Vec<GuildInfo> = ready
                .guilds
                .into_iter()
                .map(|g| GuildInfo {
                    id: g.id,
                    name: g.name,
                })
                .collect();

            println!(
                "[ok] {label}: {} ({} guilds)",
                ready.user.username,
                guilds.len()
            );

            AccountEntry {
                token,
                user_id: Some(ready.user.id),
                username: Some(ready.user.username),
                error: None,
                guilds,
            }
        }
        None => AccountEntry {
            token,
            user_id: None,
            username: None,
            error: Some("failed to reach READY".to_string()),
            guilds: Vec::new(),
        },
    }
}

fn main() -> BoxedResult<()> {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(16 * 1024 * 1024)
        .build()?
        .block_on(run())
}

async fn run() -> BoxedResult<()> {
    let build_number = discord_client_utils::find_build_numbers()
        .await?
        .client_build_number;
    println!("Using client build number {build_number}");

    let raw = tokio::fs::read_to_string(INPUT_FILE)
        .await
        .map_err(|e| format!("failed to read {INPUT_FILE}: {e}"))?;
    let tokens: Vec<String> = raw
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();

    println!("Loaded {} token(s) from {INPUT_FILE}", tokens.len());

    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let entries = Arc::new(Mutex::new(Vec::with_capacity(tokens.len())));

    let mut handles = Vec::with_capacity(tokens.len());
    for token in tokens {
        let semaphore = Arc::clone(&semaphore);
        let entries = Arc::clone(&entries);
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();
            let entry = fetch_guilds(token, build_number).await;
            entries.lock().await.push(entry);
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    let entries = std::mem::take(&mut *entries.lock().await);
    let json = serde_json::to_string_pretty(&entries)?;
    tokio::fs::write(OUTPUT_FILE, json).await?;

    println!("Wrote {} entries to {OUTPUT_FILE}", entries.len());
    Ok(())
}
