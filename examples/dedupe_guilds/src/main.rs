use discord_client_rest::rest::RestClient;
use discord_client_structs::structs::client::BuildNumbers;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const INPUT_FILE: &str = "guilds.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GuildInfo {
    id: u64,
    name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

fn pick_leaver(accounts: &[AccountEntry], members: &[usize]) -> usize {
    let mut best_pos = 0;
    let mut best_count = accounts[members[0]].guilds.len();
    for (pos, &idx) in members.iter().enumerate().skip(1) {
        let count = accounts[idx].guilds.len();
        if count > best_count {
            best_count = count;
            best_pos = pos;
        }
    }
    best_pos
}

async fn get_client<'a>(
    cache: &'a mut HashMap<String, RestClient>,
    token: &str,
    build_numbers: &BuildNumbers,
) -> BoxedResult<&'a RestClient> {
    if !cache.contains_key(token) {
        let client = RestClient::connect(
            token.to_string(),
            None,
            Some(build_numbers.clone()),
            None,
            None,
        )
        .await?;
        cache.insert(token.to_string(), client);
    }
    Ok(cache.get(token).unwrap())
}

#[tokio::main]
async fn main() -> BoxedResult<()> {
    let build_numbers = discord_client_utils::find_build_numbers().await?;

    let raw = tokio::fs::read_to_string(INPUT_FILE)
        .await
        .map_err(|e| format!("failed to read {INPUT_FILE}: {e}"))?;
    let mut accounts: Vec<AccountEntry> = serde_json::from_str(&raw)?;

    let mut membership: HashMap<u64, Vec<usize>> = HashMap::new();
    for (idx, account) in accounts.iter().enumerate() {
        for guild in &account.guilds {
            membership.entry(guild.id).or_default().push(idx);
        }
    }

    let mut duplicate_guild_ids: Vec<u64> = membership
        .iter()
        .filter(|(_, members)| members.len() > 1)
        .map(|(id, _)| *id)
        .collect();
    duplicate_guild_ids.sort_unstable();

    println!(
        "Found {} guild(s) shared by more than one account",
        duplicate_guild_ids.len()
    );

    let mut clients: HashMap<String, RestClient> = HashMap::new();

    for guild_id in duplicate_guild_ids {
        loop {
            let members = membership.get(&guild_id).unwrap();
            if members.len() <= 1 {
                break;
            }

            let pos = pick_leaver(&accounts, members);
            let acc_idx = membership.get(&guild_id).unwrap()[pos];

            let label = mask(&accounts[acc_idx].token);
            let guild_name = accounts[acc_idx]
                .guilds
                .iter()
                .find(|g| g.id == guild_id)
                .and_then(|g| g.name.clone())
                .unwrap_or_else(|| guild_id.to_string());

            let client =
                match get_client(&mut clients, &accounts[acc_idx].token, &build_numbers).await {
                    Ok(client) => client,
                    Err(e) => {
                        eprintln!("[error] {label}: failed to authenticate ({e})");
                        membership.get_mut(&guild_id).unwrap().remove(pos);
                        continue;
                    }
                };

            match client.guild(Some(guild_id)).leave().await {
                Ok(()) => {
                    accounts[acc_idx].guilds.retain(|g| g.id != guild_id);
                    membership.get_mut(&guild_id).unwrap().remove(pos);
                    println!(
                        "[left] {label} left {guild_name} ({guild_id}), now in {} guild(s)",
                        accounts[acc_idx].guilds.len()
                    );
                }
                Err(e) => {
                    eprintln!("[error] {label}: failed to leave {guild_name} ({guild_id}): {e}");
                    membership.get_mut(&guild_id).unwrap().remove(pos);
                }
            }
        }
    }

    let json = serde_json::to_string_pretty(&accounts)?;
    tokio::fs::write(INPUT_FILE, json).await?;

    println!("Updated {INPUT_FILE}");
    Ok(())
}
