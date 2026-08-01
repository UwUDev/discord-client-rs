use chrono::{DateTime, Utc};
use discord_client_rest::sessionless::SessionlessClient;
use discord_client_structs::deserializer::*;
use discord_client_structs::serializer::*;
use discord_client_structs::structs::client::BuildNumbers;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::sync::{Mutex, Semaphore};

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const INPUT_FILE: &str = "invites.txt";
const PROXIES_FILE: &str = "proxies.txt";
const DATA_FILE: &str = "invite_data.json";
const CHECKED_CACHE_FILE: &str = "checked_codes.json";
const AUTOSAVE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(30);
const MAX_CONCURRENT: usize = 48;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedInvite {
    guild_id: u64,
    code: String,
    guild_name: Option<String>,
    channel_name: Option<String>,
    approximate_member_count: Option<u32>,
    approximate_presence_count: Option<u32>,
    #[serde(deserialize_with = "deserialize_option_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_option_date_to_iso8601_string")]
    expires_at: Option<DateTime<Utc>>,
    #[serde(deserialize_with = "deserialize_iso8601_string_to_date")]
    #[serde(serialize_with = "serialize_date_to_iso8601_string")]
    checked_at: DateTime<Utc>,
}

const MAX_CODE_LEN: usize = 25;
const SNOWFLAKE_LEN: std::ops::RangeInclusive<usize> = 17..=20;

fn strip_domain_prefix(input: &str) -> &str {
    static URL_RE: OnceLock<Regex> = OnceLock::new();
    let re = URL_RE.get_or_init(|| {
        Regex::new(r"(?i)^(?:https?://)?(?:www\.)?(?:discord\.gg/|discord(?:app)?\.com/invite/)")
            .unwrap()
    });

    match re.find(input) {
        Some(m) => &input[m.end()..],
        None => input
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .trim_start_matches("www."),
    }
}

fn has_snowflake_suffix(code: &str) -> bool {
    match code.rsplit_once('-') {
        Some((_, id)) => {
            SNOWFLAKE_LEN.contains(&id.len()) && id.chars().all(|c| c.is_ascii_digit())
        }
        None => false,
    }
}

fn extract_invite_code(line: &str) -> Result<String, &'static str> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("empty line");
    }

    let rest = strip_domain_prefix(trimmed);
    let end = rest.find(['/', ' ', '\t']).unwrap_or(rest.len());
    let candidate = &rest[..end];

    if candidate.is_empty() {
        return Err("no code found");
    }
    if candidate.contains('?') {
        return Err("contains query parameters");
    }
    if !candidate
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        return Err("contains invalid characters");
    }
    if candidate.len() > MAX_CODE_LEN && !has_snowflake_suffix(candidate) {
        return Err("code too long");
    }

    Ok(candidate.to_string())
}

fn load_proxies() -> Vec<String> {
    std::fs::read_to_string(PROXIES_FILE)
        .map(|raw| {
            raw.lines()
                .map(str::trim)
                .filter(|line| !line.is_empty() && !line.starts_with('#'))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn load_checked_codes() -> BoxedResult<std::collections::HashSet<String>> {
    match std::fs::read_to_string(CHECKED_CACHE_FILE) {
        Ok(raw) => Ok(serde_json::from_str(&raw)
            .map_err(|e| format!("failed to parse {CHECKED_CACHE_FILE}: {e}"))?),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Default::default()),
        Err(e) => Err(format!("failed to read {CHECKED_CACHE_FILE}: {e}").into()),
    }
}

fn save_checked_codes(codes: &std::collections::HashSet<String>) -> BoxedResult<()> {
    let json = serde_json::to_string(codes)
        .map_err(|e| format!("failed to serialize {CHECKED_CACHE_FILE}: {e}"))?;

    let tmp_path = format!("{CHECKED_CACHE_FILE}.tmp");
    std::fs::write(&tmp_path, json).map_err(|e| format!("failed to write {tmp_path}: {e}"))?;
    std::fs::rename(&tmp_path, CHECKED_CACHE_FILE)
        .map_err(|e| format!("failed to replace {CHECKED_CACHE_FILE}: {e}"))?;

    Ok(())
}

fn is_definitively_invalid(err: &BoxedError) -> bool {
    err.to_string().contains("failed with code 404")
}

fn load_saved() -> BoxedResult<Vec<SavedInvite>> {
    match std::fs::read_to_string(DATA_FILE) {
        Ok(raw) => {
            Ok(serde_json::from_str(&raw)
                .map_err(|e| format!("failed to parse {DATA_FILE}: {e}"))?)
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Vec::new()),
        Err(e) => Err(format!("failed to read {DATA_FILE}: {e}").into()),
    }
}

fn save(entries: &[SavedInvite]) -> BoxedResult<()> {
    let json = serde_json::to_string_pretty(entries)
        .map_err(|e| format!("failed to serialize {DATA_FILE}: {e}"))?;

    let tmp_path = format!("{DATA_FILE}.tmp");
    std::fs::write(&tmp_path, json).map_err(|e| format!("failed to write {tmp_path}: {e}"))?;
    std::fs::rename(&tmp_path, DATA_FILE)
        .map_err(|e| format!("failed to replace {DATA_FILE}: {e}"))?;

    Ok(())
}

fn is_better(candidate: &SavedInvite, current: &SavedInvite) -> bool {
    match (candidate.expires_at, current.expires_at) {
        (Some(a), Some(b)) => a >= b,
        (None, _) => true,
        (Some(_), None) => false,
    }
}

fn upsert(entries: &mut Vec<SavedInvite>, candidate: SavedInvite) {
    match entries
        .iter_mut()
        .find(|e| e.guild_id == candidate.guild_id)
    {
        Some(current) => {
            if is_better(&candidate, current) {
                *current = candidate;
            }
        }
        None => entries.push(candidate),
    }
}

async fn check_invite(
    proxy: Option<String>,
    build_numbers: BuildNumbers,
    code: String,
    entries: Arc<Mutex<Vec<SavedInvite>>>,
    checked_codes: Arc<Mutex<std::collections::HashSet<String>>>,
    valid: Arc<AtomicUsize>,
    invalid: Arc<AtomicUsize>,
    queued_for: std::time::Duration,
) {
    let proxy = match proxy {
        None => None,
        Some(p) => {
            let session = rand::random_range(100000..999999);
            let p = p.replace("{session}", &session.to_string());
            Some(p)
        }
    };

    let connect_start = std::time::Instant::now();
    let client =
        match SessionlessClient::connect(None, Some(build_numbers), None, None, proxy, false).await
        {
            Ok(client) => client,
            Err(e) => {
                invalid.fetch_add(1, Ordering::Relaxed);
                println!(
                    "[invalid] {code}: failed to connect after {:?} (queued {:?}): {e}",
                    connect_start.elapsed(),
                    queued_for
                );
                return;
            }
        };
    let connect_elapsed = connect_start.elapsed();

    let fetch_start = std::time::Instant::now();
    match client.invite().get_invite(code.clone()).await {
        Ok(invite) => {
            checked_codes.lock().await.insert(code.clone());

            let Some(guild_id) = invite.guild_id else {
                println!("[skipped] {code}: not a guild invite");
                return;
            };

            valid.fetch_add(1, Ordering::Relaxed);
            let guild_name = invite.guild.as_ref().map(|g| g.name.clone());
            println!(
                "[valid] {code}: guild={:?} channel={:?} members={:?} online={:?} expires_at={:?} \
                 (queued {:?}, connect {:?}, fetch {:?})",
                guild_name,
                invite.channel.as_ref().and_then(|c| c.name.clone()),
                invite.approximate_member_count,
                invite.approximate_presence_count,
                invite.expires_at,
                queued_for,
                connect_elapsed,
                fetch_start.elapsed(),
            );

            let candidate = SavedInvite {
                guild_id,
                code,
                guild_name,
                channel_name: invite.channel.and_then(|c| c.name),
                approximate_member_count: invite.approximate_member_count,
                approximate_presence_count: invite.approximate_presence_count,
                expires_at: invite.expires_at,
                checked_at: Utc::now(),
            };

            let mut entries = entries.lock().await;
            upsert(&mut entries, candidate);
        }
        Err(e) => {
            invalid.fetch_add(1, Ordering::Relaxed);
            if is_definitively_invalid(&e) {
                checked_codes.lock().await.insert(code.clone());
            }
            println!(
                "[invalid] {code}: {e} (queued {:?}, connect {:?}, fetch {:?})",
                queued_for,
                connect_elapsed,
                fetch_start.elapsed(),
            );
        }
    }
}

#[tokio::main]
async fn main() -> BoxedResult<()> {
    pretty_env_logger::init();

    let raw = std::fs::read_to_string(INPUT_FILE)
        .map_err(|e| format!("failed to read {INPUT_FILE}: {e}"))?;
    let codes: Vec<String> = raw
        .lines()
        .filter(|line| !line.trim().is_empty())
        .filter_map(|line| match extract_invite_code(line) {
            Ok(code) => Some(code),
            Err(reason) => {
                println!("[skipped] {line}: {reason}");
                None
            }
        })
        .collect();

    println!("Loaded {} invite code(s) from {INPUT_FILE}", codes.len());

    let saved = load_saved()?;
    let checked_codes = load_checked_codes()?;
    let codes: Vec<String> = codes
        .into_iter()
        .filter(|code| !checked_codes.contains(code))
        .collect();

    println!(
        "Skipping {} already-checked code(s) ({} loaded from {DATA_FILE}), {} left to check",
        checked_codes.len(),
        saved.len(),
        codes.len()
    );

    let proxies = load_proxies();
    let proxy_pool: Vec<Option<String>> = if proxies.is_empty() {
        println!("No {PROXIES_FILE} found (or it's empty); running without a proxy");
        vec![None]
    } else {
        println!(
            "Loaded {} proxy(ies) from {PROXIES_FILE}; each check connects its own client to rotate",
            proxies.len()
        );
        proxies.into_iter().map(Some).collect()
    };

    let build_numbers = discord_client_utils::find_build_numbers()
        .await
        .map_err(|e| format!("failed to fetch build numbers: {e}"))?;

    let next_proxy = Arc::new(AtomicUsize::new(0));

    let entries = Arc::new(Mutex::new(saved));
    let checked_codes = Arc::new(Mutex::new(checked_codes));
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT));
    let valid = Arc::new(AtomicUsize::new(0));
    let invalid = Arc::new(AtomicUsize::new(0));

    let autosave_entries = Arc::clone(&entries);
    let autosave_checked = Arc::clone(&checked_codes);
    let autosave_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(AUTOSAVE_INTERVAL);
        interval.tick().await; // first tick fires immediately, skip it
        loop {
            interval.tick().await;
            let entries = autosave_entries.lock().await;
            if let Err(e) = save(&entries) {
                eprintln!("autosave: failed to save {DATA_FILE}: {e}");
            }
            drop(entries);
            let checked = autosave_checked.lock().await;
            if let Err(e) = save_checked_codes(&checked) {
                eprintln!("autosave: failed to save {CHECKED_CACHE_FILE}: {e}");
            }
            drop(checked);
        }
    });

    let mut handles = Vec::with_capacity(codes.len());
    for code in codes {
        let idx = next_proxy.fetch_add(1, Ordering::Relaxed) % proxy_pool.len();
        let proxy = proxy_pool[idx].clone();
        let build_numbers = build_numbers.clone();
        let entries = Arc::clone(&entries);
        let checked_codes = Arc::clone(&checked_codes);
        let semaphore = Arc::clone(&semaphore);
        let valid = Arc::clone(&valid);
        let invalid = Arc::clone(&invalid);

        handles.push(tokio::spawn(async move {
            let wait_start = std::time::Instant::now();
            let _permit = semaphore
                .acquire_owned()
                .await
                .expect("semaphore was not closed");
            let queued_for = wait_start.elapsed();
            check_invite(
                proxy,
                build_numbers,
                code,
                entries,
                checked_codes,
                valid,
                invalid,
                queued_for,
            )
            .await;
        }));
    }

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);

    let mut interrupted = false;
    let mut remaining = handles.into_iter();

    while let Some(handle) = remaining.next() {
        let abort_handle = handle.abort_handle();
        tokio::select! {
            _ = &mut ctrl_c => {
                println!("\nReceived Ctrl+C, stopping and saving progress...");
                interrupted = true;
                abort_handle.abort();
                break;
            }
            res = handle => {
                if let Err(e) = res {
                    if !e.is_cancelled() {
                        eprintln!("a check task panicked: {e}");
                    }
                }
            }
        }
    }

    if interrupted {
        for handle in remaining {
            handle.abort();
        }
    }

    autosave_handle.abort();

    let entries = entries.lock().await;
    save(&entries)?;
    let checked_codes = checked_codes.lock().await;
    save_checked_codes(&checked_codes)?;

    println!(
        "{} {} valid, {} invalid. {} saved to {DATA_FILE}, {} codes cached in {CHECKED_CACHE_FILE}.",
        if interrupted { "Interrupted." } else { "Done." },
        valid.load(Ordering::Relaxed),
        invalid.load(Ordering::Relaxed),
        entries.len(),
        checked_codes.len(),
    );

    Ok(())
}
