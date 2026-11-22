use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use discord_client_rest::age_gate::AgeGatedError;
use discord_client_rest::rest::{RequestProperties, RestClient};
use discord_client_structs::structs::client::ClientSession;
use serde_json::Value;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, Semaphore};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const CAPABILITIES: u32 = 53_607_934;
const MAX_CONCURRENT: usize = 4;
const INPUT_FILE: &str = "tokens.txt";
const READY_TIMEOUT: Duration = Duration::from_secs(15);

// public NSFW channel used as the age probe.
const PROBE_CHANNEL_ID: u64 = 1566574633459781663;

#[derive(Debug, Clone, Copy)]
enum Verdict {
    Adult,
    Minor,
}

struct Output {
    adult: Mutex<File>,
    minor: Mutex<File>,
    unknown: Mutex<File>,
}

impl Output {
    async fn open() -> BoxedResult<Self> {
        async fn create(path: &str) -> BoxedResult<Mutex<File>> {
            Ok(Mutex::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(path)
                    .await?,
            ))
        }
        Ok(Self {
            adult: create("adult.txt").await?,
            minor: create("minor.txt").await?,
            unknown: create("unknown.txt").await?,
        })
    }

    async fn append(file: &Mutex<File>, token: &str) {
        let mut file = file.lock().await;
        let _ = file.write_all(format!("{token}\n").as_bytes()).await;
        let _ = file.flush().await;
    }
}

fn mask(token: &str) -> String {
    if token.len() <= 12 {
        return "*".repeat(token.len());
    }
    format!("{}...{}", &token[..8], &token[token.len() - 4..])
}

async fn probe(rest: &RestClient) -> BoxedResult<Verdict> {
    let query = [("limit".to_string(), "10".to_string())].into();
    let result: BoxedResult<Value> = rest
        .get(
            &format!("channels/{PROBE_CHANNEL_ID}/messages"),
            Some(query),
            Some(RequestProperties::home()),
        )
        .await;

    match result {
        Ok(_) => Ok(Verdict::Adult),
        Err(e) if e.downcast_ref::<AgeGatedError>().is_some() => Ok(Verdict::Minor),
        Err(e) => Err(e),
    }
}

async fn check(rest: &RestClient, gateway: &mut GatewayClient) -> BoxedResult<Verdict> {
    loop {
        match tokio::time::timeout(READY_TIMEOUT, gateway.next_event()).await {
            Ok(Ok(Event::Ready(_))) => break,
            Ok(Ok(_)) => continue,
            Ok(Err(e)) => return Err(e),
            Err(_) => return Err("timed out waiting for READY".into()),
        }
    }

    let probe_fut = probe(rest);
    tokio::pin!(probe_fut);

    loop {
        tokio::select! {
            result = &mut probe_fut => return result,
            event = gateway.next_event() => {
                event?;
            }
        }
    }
}

async fn check_token(token: String, out: Arc<Output>) {
    let label = mask(&token);

    let rest = match RestClient::connect(
        token.clone(),
        None,
        None,
        Some(ClientSession::new()),
        None,
    )
    .await
    {
        Ok(rest) => rest,
        Err(e) => {
            eprintln!("[unknown] {label}: {e}");
            Output::append(&out.unknown, &token).await;
            return;
        }
    };

    let mut gateway = match GatewayClient::connect(
        token.clone(),
        false,
        CAPABILITIES,
        Some(rest.build_numbers.client_build_number),
    )
    .await
    {
        Ok(gateway) => gateway,
        Err(e) => {
            eprintln!("[unknown] {label}: gateway: {e}");
            Output::append(&out.unknown, &token).await;
            return;
        }
    };

    let result = check(&rest, &mut gateway).await;
    let _ = gateway.graceful_shutdown().await;

    match result {
        Ok(Verdict::Adult) => {
            println!("[adult] {label}");
            Output::append(&out.adult, &token).await;
        }
        Ok(Verdict::Minor) => {
            println!("[minor] {label}");
            Output::append(&out.minor, &token).await;
        }
        Err(e) => {
            eprintln!("[unknown] {label}: {e}");
            Output::append(&out.unknown, &token).await;
        }
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
    let raw = tokio::fs::read_to_string(INPUT_FILE)
        .await
        .map_err(|e| format!("failed to read {INPUT_FILE}: {e}"))?;
    let tokens: Vec<String> = raw
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .map(str::to_string)
        .collect();
    println!("Loaded {} token(s) from {INPUT_FILE}", tokens.len());

    let out = Arc::new(Output::open().await?);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));

    let mut handles = Vec::with_capacity(tokens.len());
    for token in tokens {
        let semaphore = Arc::clone(&semaphore);
        let out = Arc::clone(&out);
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();
            check_token(token, out).await;
        }));
    }
    for h in handles {
        let _ = h.await;
    }

    println!("Done.");
    Ok(())
}
