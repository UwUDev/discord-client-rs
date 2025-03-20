use serde_json::{Value, json};
use std::str::FromStr;

pub(crate) fn create_op_37(guild_ids: Vec<u64>) -> String {
    let mut payload = Value::from_str(r#"{"op":37,"d":{"subscriptions":{}}}"#).unwrap();
    let guild_payload = Value::from_str(
        r#"{"typing":true,"threads":true,"activities":true,"member_updates":true}"#,
    )
    .unwrap();

    for guild_id in guild_ids {
        payload["d"]["subscriptions"]
            .as_object_mut()
            .unwrap()
            .insert(guild_id.to_string(), guild_payload.clone());
    }

    payload.to_string()
}

pub(crate) fn create_op_36(guild_id: u64) -> String {
    let payload = json!({
        "op": 36,
        "d": {
            "guild_id": guild_id
        }
    });

    payload.to_string()
}

pub(crate) fn create_op_34(guild_id: u64, channel_ids: Vec<u64>) -> String {
    let mut payload = Value::from_str(r#"{"op":34,"d":{"guild_id":0,"channel_ids":[]}}"#).unwrap();
    payload["d"]["guild_id"] = Value::from(guild_id);
    payload["d"]["channel_ids"] = Value::from(channel_ids);

    payload.to_string()
}
