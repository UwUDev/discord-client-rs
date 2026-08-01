use crate::BoxedResult;
use crate::rate_limit::RateLimitError;
use log::warn;
use serde_json::Value;
use std::time::Duration;

const CLOUDFLARE_BLOCK_RETRY_AFTER: Duration = Duration::from_secs(2);

pub(crate) fn parse_error_body(bytes: &[u8], status: u16, url: &str) -> BoxedResult<Value> {
    serde_json::from_slice(bytes).map_err(|_| {
        let preview: String = String::from_utf8_lossy(bytes).chars().take(200).collect();
        format!(
            "Request to {} failed with code {} and a non-JSON body (likely a Cloudflare block \
             rather than a Discord API response): {}",
            url, status, preview
        )
        .into()
    })
}

pub(crate) fn rate_limit_from_body(bytes: &[u8], url: &str) -> RateLimitError {
    match serde_json::from_slice::<Value>(bytes) {
        Ok(json) => {
            let retry_after_secs = json["retry_after"].as_f64().unwrap_or(1.0);
            let global = json["global"].as_bool().unwrap_or(false);
            RateLimitError::new(Duration::from_secs_f64(retry_after_secs), global)
        }
        Err(_) => {
            warn!(
                "Non-JSON 429 response from {} (likely a Cloudflare edge rate limit, e.g. from a \
                 non-rotating proxy); backing off {:?} and retrying",
                url, CLOUDFLARE_BLOCK_RETRY_AFTER
            );
            RateLimitError::new(CLOUDFLARE_BLOCK_RETRY_AFTER, false)
        }
    }
}
