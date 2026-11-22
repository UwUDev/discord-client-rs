use std::error::Error;
use std::fmt::{Display, Formatter};

pub const AGE_GATED_CODE: i64 = 50269;

#[derive(Debug, Clone)]
pub struct AgeGatedError {
    pub message: String,
    pub code: i64,
}

impl Display for AgeGatedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Age-gated content ({}): {}", self.code, self.message)
    }
}

impl Error for AgeGatedError {}
