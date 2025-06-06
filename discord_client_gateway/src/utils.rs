use discord_client_structs::structs::user::activity::Activity;
use discord_client_structs::structs::user::status::StatusType;
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

pub(crate) fn create_op_35(
    guild_id: u64,
    query: &str,
    continuation_token: Option<u64>,
    nonce: Option<&str>,
) -> String {
    let mut payload =
        Value::from_str(r#"{"op":35,"d":{"guild_id":0,"query":"","continuation_token":null}}"#)
            .unwrap();
    payload["d"]["guild_id"] = Value::from(guild_id);
    payload["d"]["query"] = Value::from(query);
    payload["d"]["continuation_token"] = match continuation_token {
        Some(token) => Value::from(token.to_string()),
        None => Value::Null,
    };
    payload["d"]["nonce"] = match nonce {
        Some(nonce) => Value::from(nonce),
        None => Value::Null,
    };

    payload.to_string()
}

pub(crate) fn create_op_8(
    guild_ids: u64,
    query: Option<&str>,
    limit: Option<u64>,
    presences: Option<bool>,
    user_ids: Option<Vec<u64>>,
    nonce: Option<&str>,
) -> String {
    use serde_json::{Map, Value, json};

    let mut d = Map::new();

    // I tried to put multiple guild_ids, but it didn't work, it just returned the first one
    let guild_ids_str: Vec<String> = vec![guild_ids.to_string()];
    d.insert("guild_id".to_string(), Value::from(guild_ids_str));

    let query_value = query.unwrap_or("");
    d.insert("query".to_string(), Value::from(query_value));

    if query_value.is_empty() {
        d.insert("limit".to_string(), json!(0));
    } else if let Some(l) = limit {
        d.insert("limit".to_string(), json!(l));
    }

    if let Some(p) = presences {
        d.insert("presences".to_string(), json!(p));
    }

    if let Some(ids) = &user_ids {
        let user_ids_str: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        d.insert("user_ids".to_string(), json!(user_ids_str));
    }

    if let Some(n) = nonce {
        d.insert("nonce".to_string(), json!(n));
    }

    json!({
        "op": 8,
        "d": d
    })
    .to_string()
}

pub(crate) fn create_op_29<T: serde::Serialize>(target_session_id: &str, payload: T) -> String {
    let payload = json!({
        "op": 29,
        "d": {
            "target_session_id": target_session_id,
            "payload": payload
        }
    });
    payload.to_string()
}

pub(crate) fn create_op_6(token: &str, session_id: &str, last_seq: u32) -> String {
    let payload = json!({
        "op": 6,
        "d": {
            "token": token,
            "session_id": session_id,
            "seq": last_seq
        }
    });
    payload.to_string()
}

pub(crate) fn create_op_31(guild_ids: Vec<u64>) -> String {
    let guild_ids_str: Vec<String> = guild_ids.iter().map(|id| id.to_string()).collect();
    let payload = json!({
        "op": 31,
        "d": {
            "guild_ids": guild_ids_str
        }
    });
    payload.to_string()
}

pub(crate) fn create_op_3(
    since: u64,
    activities: Vec<Activity>,
    status: StatusType,
    afk: bool,
) -> String {
    let payload = json!({
        "op": 3,
        "d": {
            "since": since,
            "activities": activities,
            "status": status.as_str(),
            "afk": afk
        }
    });
    payload.to_string()
}
