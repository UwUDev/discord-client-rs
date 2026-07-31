use discord_client_gateway::events::Event;
use discord_client_gateway::gateway::GatewayClient;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::sync::{Mutex, Semaphore};
use tokio::time::Instant;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const CAPABILITIES: u32 = 53_607_934;
const MAX_CONCURRENT: usize = 8;
const INPUT_FILE: &str = "tokens.txt";
const READY_TIMEOUT: Duration = Duration::from_secs(15);
const POST_READY_WINDOW: Duration = Duration::from_secs(5);

#[derive(Debug, Clone, Copy)]
enum Bucket {
    Valid,
    Dead,
    CaptchaLocked,
    MailLocked,
    PhoneLocked,
}

enum Outcome {
    Valid,
    Dead(String),
    Locked(Bucket),
}

struct OutputFiles {
    valid: Mutex<File>,
    dead: Mutex<File>,
    captcha_locked: Mutex<File>,
    mail_locked: Mutex<File>,
    phone_locked: Mutex<File>,
}

impl OutputFiles {
    async fn open() -> BoxedResult<Self> {
        async fn create(path: &str) -> BoxedResult<File> {
            Ok(OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .await?)
        }

        Ok(Self {
            valid: Mutex::new(create("valid.txt").await?),
            dead: Mutex::new(create("dead.txt").await?),
            captcha_locked: Mutex::new(create("captcha_locked.txt").await?),
            mail_locked: Mutex::new(create("mail_locked.txt").await?),
            phone_locked: Mutex::new(create("phone_locked.txt").await?),
        })
    }

    async fn append(&self, bucket: Bucket, token: &str) {
        let file = match bucket {
            Bucket::Valid => &self.valid,
            Bucket::Dead => &self.dead,
            Bucket::CaptchaLocked => &self.captcha_locked,
            Bucket::MailLocked => &self.mail_locked,
            Bucket::PhoneLocked => &self.phone_locked,
        };

        let mut file = file.lock().await;
        let line = format!("{token}\n");
        let _ = file.write_all(line.as_bytes()).await;
        let _ = file.flush().await;
    }
}

fn classify_required_action(action: &str) -> Bucket {
    let upper = action.to_ascii_uppercase();
    if upper.contains("CAPTCHA") {
        Bucket::CaptchaLocked
    } else if upper.contains("PHONE") {
        Bucket::PhoneLocked
    } else if upper.contains("EMAIL") {
        Bucket::MailLocked
    } else {
        eprintln!("note: unrecognized required_action value {action:?}, treating as valid");
        Bucket::Valid
    }
}

fn mask(token: &str) -> String {
    if token.len() <= 12 {
        return "*".repeat(token.len());
    }
    format!("{}...{}", &token[..8], &token[token.len() - 4..])
}

async fn wait_for_outcome(client: &mut GatewayClient) -> Outcome {
    let mut required_action = loop {
        match tokio::time::timeout(READY_TIMEOUT, client.next_event()).await {
            Ok(Ok(Event::Ready(ready))) => break ready.required_action,
            Ok(Ok(_other)) => continue,
            Ok(Err(e)) => return Outcome::Dead(e.to_string()),
            Err(_) => return Outcome::Dead("timed out waiting for READY".into()),
        }
    };

    let deadline = Instant::now() + POST_READY_WINDOW;
    while required_action.is_none() {
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }

        match tokio::time::timeout(remaining, client.next_event()).await {
            Ok(Ok(Event::UserRequiredActionUpdate(update))) => {
                required_action = update.required_action;
            }
            Ok(Ok(_other)) => continue,
            Ok(Err(_)) | Err(_) => break,
        }
    }

    match required_action {
        None => Outcome::Valid,
        Some(action) => Outcome::Locked(classify_required_action(&action)),
    }
}

async fn check_token(token: String, build_number: u32, files: Arc<OutputFiles>) {
    let label = mask(&token);

    match GatewayClient::connect(token.clone(), false, CAPABILITIES, Some(build_number)).await {
        Err(e) => {
            eprintln!("[dead] {label}: {e}");
            files.append(Bucket::Dead, &token).await;
        }
        Ok(mut client) => {
            let outcome = wait_for_outcome(&mut client).await;
            let _ = client.graceful_shutdown().await;

            match outcome {
                Outcome::Dead(reason) => {
                    eprintln!("[dead] {label}: {reason}");
                    files.append(Bucket::Dead, &token).await;
                }
                Outcome::Valid => {
                    println!("[valid] {label}");
                    files.append(Bucket::Valid, &token).await;
                }
                Outcome::Locked(bucket) => {
                    println!("[locked] {label}: {bucket:?}");
                    files.append(bucket, &token).await;
                }
            }
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

    let files = Arc::new(OutputFiles::open().await?);
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));

    let mut handles = Vec::with_capacity(tokens.len());
    for token in tokens {
        let semaphore = Arc::clone(&semaphore);
        let files = Arc::clone(&files);
        handles.push(tokio::spawn(async move {
            let _permit = semaphore.acquire_owned().await.unwrap();
            check_token(token, build_number, files).await;
        }));
    }

    for handle in handles {
        let _ = handle.await;
    }

    println!("Done.");
    Ok(())
}
