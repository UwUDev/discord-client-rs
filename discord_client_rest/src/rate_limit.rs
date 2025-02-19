use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tokio::time::{Duration, Instant};

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
    notify: Arc<Notify>,
    pub(crate) route_mutex: Arc<Mutex<()>>,
}

impl RateLimiter {
    pub(crate) fn new() -> Self {
        RateLimiter {
            retry_until: Arc::new(Mutex::new(None)),
            notify: Arc::new(Notify::new()),
            route_mutex: Arc::new(Mutex::new(())),
        }
    }

    pub(crate) async fn wait_if_needed(&self) {
        loop {
            let now = Instant::now();
            let retry_time = {
                let retry_until = self.retry_until.lock().await;
                *retry_until
            };

            if let Some(time) = retry_time {
                if time > now {
                    let duration = time - now;
                    tokio::select! {
                        _ = tokio::time::sleep(duration) => {},
                        _ = self.notify.notified() => {},
                    }
                } else {
                    let mut retry_until = self.retry_until.lock().await;
                    *retry_until = None;
                    return;
                }
            } else {
                return;
            }
        }
    }

    pub(crate) async fn update(&self, retry_after: Duration) {
        let mut retry_until = self.retry_until.lock().await;
        let new_retry_until = Instant::now() + retry_after;
        *retry_until = Some(new_retry_until);
        self.notify.notify_waiters();
    }
}
