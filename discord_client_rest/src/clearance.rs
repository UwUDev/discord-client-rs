use crate::BoxedResult;
use log::warn;
use rquest::Client;

pub(crate) async fn get_invisible(client: &Client) -> BoxedResult<(String, String)> {
    let response = client
        .get("https://discord.com/cdn-cgi/challenge-platform/scripts/jsd/main.js")
        .send()
        .await?;
    let text = response.text().await?;
    let re = regex::Regex::new(r"0\.\d+:\d+:[a-zA-Z0-9-_]+").unwrap();
    let s = re
        .captures(&text)
        .unwrap()
        .get(0)
        .unwrap()
        .as_str()
        .to_string();
    let js = text.split("'.split(','),").collect::<Vec<&str>>()[0]
        .split("='")
        .last()
        .unwrap();

    let mut key = String::new();

    for elem in js.split(",") {
        if elem.len() == 65 {
            key = elem.to_string();
        }
    }

    Ok((key, s))
}

pub(crate) async fn get_clearance_cookie(
    _client: &Client,
    _r: String,
    _t: String,
    key: String,
    _s: String,
) -> BoxedResult<()> {
    let _wp = generate_wp(key);
    warn!(
        "Cloudflare clearance cookie is not implemented yet and can result in getting captcha/flagged"
    );

    // todo: final request to get the clearance cookie

    Ok(())
}

fn generate_wp(_key: String) -> String {
    // todo: implement this pile of garbage in rust https://github.com/0rl4ndo/CloudFlareX/blob/main/server/index.js
    String::new()
}
