use base64::Engine as _;
use base64::engine::{GeneralPurpose, GeneralPurposeConfig};
use chrono::{DateTime, TimeZone, Utc};
use std::error::Error;

pub fn parse_id_from_token(token: &str) -> Result<u64, Box<dyn Error>> {
    let alphabet = base64::alphabet::STANDARD;
    let custom_engine = GeneralPurpose::new(
        &alphabet,
        GeneralPurposeConfig::new().with_decode_allow_trailing_bits(true),
    );

    let parts: Vec<&str> = token.split('.').collect();

    if parts.is_empty() {
        return Err("Invalid token format".into());
    }

    let first_part = parts[0];
    let padding_needed = (4 - (first_part.len() % 4)) % 4;
    let padded_part = format!("{}{}", first_part, "=".repeat(padding_needed));

    let decoded = custom_engine.decode(padded_part.as_bytes())?;
    let id_str = String::from_utf8(decoded)?;
    Ok(id_str.parse()?)
}

pub fn parse_date_from_id(id: u64) -> Option<DateTime<Utc>> {
    let timestamp = (id >> 22) + 1420070400000;
    Utc.timestamp_millis_opt(timestamp as i64).single()
}
