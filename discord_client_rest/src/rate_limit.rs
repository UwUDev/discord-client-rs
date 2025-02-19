use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{Duration, Instant, sleep};

pub struct RateLimitError {
    pub retry_after: Duration,
    pub global: bool,
}

impl RateLimitError {
    pub fn new(retry_after: Duration, global: bool) -> Self {
        Self {
            retry_after,
            global,
        }
    }
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Rate limit error: retry_after: {:?}, global: {}",
            self.retry_after, self.global
        )
    }
}

impl Debug for RateLimitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RateLimitError {{ retry_after: {:?}, global: {} }}",
            self.retry_after, self.global
        )
    }
}

impl std::error::Error for RateLimitError {}

#[derive(Clone)]
pub(crate) struct RateLimiter {
    retry_until: Arc<Mutex<Option<Instant>>>,
    is_global: Arc<Mutex<bool>>,
}

impl RateLimiter {
    pub(crate) fn new() -> Self {
        RateLimiter {
            retry_until: Arc::new(Mutex::new(None)),
            is_global: Arc::new(Mutex::new(false)),
        }
    }

    pub(crate) async fn wait_if_needed(&self) {
        if let Some(retry_time) = *self.retry_until.lock().await {
            let now = Instant::now();
            if retry_time > now {
                let wait_duration = retry_time - now;
                sleep(wait_duration).await;
            }
        }
    }

    pub(crate) async fn update(&self, retry_after: Duration, global: bool) {
        let mut retry_until = self.retry_until.lock().await;
        *retry_until = Some(Instant::now() + retry_after);

        let mut is_global = self.is_global.lock().await;
        *is_global = global;
    }
}
