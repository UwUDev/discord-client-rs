use base64::{Engine as _, engine::general_purpose::STANDARD};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
struct SuperProps {
    os: String,
    browser: String,
    device: String,
    system_locale: String,
    has_client_mods: bool,
    browser_user_agent: String,
    browser_version: String,
    os_version: String,
    referrer: String,
    referring_domain: String,
    referrer_current: String,
    referring_domain_current: String,
    release_channel: String,
    client_build_number: u32,
    client_event_source: Option<String>,
}

pub fn build_super_props(build_num: u32) -> String {
    let props = SuperProps {
        os: "Windows".to_string(),
        browser: "Chrome".to_string(),
        device: "".to_string(),
        system_locale: "en-US".to_string(),
        has_client_mods: false,
        browser_user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".to_string(),
        browser_version: "133.0.0.0".to_string(),
        os_version: "10".to_string(),
        referrer: "".to_string(),
        referring_domain: "".to_string(),
        referrer_current: "".to_string(),
        referring_domain_current: "".to_string(),
        release_channel: "stable".to_string(),
        client_build_number: build_num,
        client_event_source: None,
    };

    let json_string = serde_json::to_string(&props).unwrap();
    STANDARD.encode(json_string)
}
