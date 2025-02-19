use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use std::error::Error;

pub fn parse_id_from_token(token: &str) -> Result<u64, Box<dyn Error>> {
    let parts: Vec<&str> = token.split('.').collect();
    let id_str = BASE64.decode(parts[0])?;
    let id_str = String::from_utf8(id_str)?;
    Ok(id_str.parse()?)
}
