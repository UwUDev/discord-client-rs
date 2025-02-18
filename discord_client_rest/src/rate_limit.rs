use std::fmt::{Debug, Formatter};
use std::time::Duration;

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
