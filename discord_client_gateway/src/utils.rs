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

pub(crate) fn create_op_4(
    guild_id: Option<u64>,
    channel_id: Option<u64>,
    self_mute: bool,
    self_deaf: bool,
    self_video: Option<bool>,
) -> String {
    use serde_json::Map;

    let mut d = Map::new();
    d.insert(
        "guild_id".to_string(),
        guild_id
            .map(|g| Value::from(g.to_string()))
            .unwrap_or(Value::Null),
    );
    d.insert(
        "channel_id".to_string(),
        channel_id
            .map(|c| Value::from(c.to_string()))
            .unwrap_or(Value::Null),
    );
    d.insert("self_mute".to_string(), json!(self_mute));
    d.insert("self_deaf".to_string(), json!(self_deaf));
    if let Some(v) = self_video {
        d.insert("self_video".to_string(), json!(v));
    }

    json!({ "op": 4, "d": d }).to_string()
}

pub(crate) fn create_op_18(
    stream_type: &str,
    guild_id: Option<u64>,
    channel_id: u64,
    preferred_region: Option<&str>,
) -> String {
    use serde_json::Map;

    let mut d = Map::new();
    d.insert("type".to_string(), json!(stream_type));
    d.insert(
        "guild_id".to_string(),
        guild_id
            .map(|g| Value::from(g.to_string()))
            .unwrap_or(Value::Null),
    );
    d.insert(
        "channel_id".to_string(),
        Value::from(channel_id.to_string()),
    );
    d.insert(
        "preferred_region".to_string(),
        preferred_region.map(Value::from).unwrap_or(Value::Null),
    );

    json!({ "op": 18, "d": d }).to_string()
}

pub(crate) fn create_stream_key_op(op: u8, stream_key: &str) -> String {
    json!({ "op": op, "d": { "stream_key": stream_key } }).to_string()
}

pub(crate) fn create_op_22(stream_key: &str, paused: bool) -> String {
    json!({ "op": 22, "d": { "stream_key": stream_key, "paused": paused } }).to_string()
}

pub(crate) fn create_op_41(
    initialization_timestamp: u64,
    session_id: &str,
    client_launch_id: &str,
) -> String {
    json!({
        "op": 41,
        "d": {
            "initialization_timestamp": initialization_timestamp,
            "session_id": session_id,
            "client_launch_id": client_launch_id
        }
    })
    .to_string()
}

pub(crate) fn create_op_5() -> String {
    json!({ "op": 5, "d": null }).to_string()
}

pub(crate) fn create_op_13(channel_id: u64) -> String {
    json!({ "op": 13, "d": { "channel_id": channel_id.to_string() } }).to_string()
}

pub(crate) fn create_op_32(preferred_region: Option<&str>) -> String {
    json!({
        "op": 32,
        "d": { "preferred_region": preferred_region.map(Value::from).unwrap_or(Value::Null) }
    })
    .to_string()
}

pub(crate) fn create_op_33() -> String {
    json!({ "op": 33, "d": {} }).to_string()
}

pub(crate) fn create_op_38(guild_id: u64, obfuscated_channel_ids: Vec<u64>) -> String {
    let ids: Vec<String> = obfuscated_channel_ids
        .iter()
        .map(|id| id.to_string())
        .collect();
    json!({
        "op": 38,
        "d": { "guild_id": guild_id.to_string(), "obfuscated_channel_ids": ids }
    })
    .to_string()
}

pub(crate) fn create_op_39(guild_id: u64, channel_id: u64) -> String {
    json!({
        "op": 39,
        "d": { "guild_id": guild_id.to_string(), "channel_id": channel_id.to_string() }
    })
    .to_string()
}

pub(crate) fn create_op_40(
    seq: Option<u32>,
    qos: Value,
    ver: u32,
    active: bool,
    reasons: Vec<String>,
) -> String {
    json!({
        "op": 40,
        "d": { "seq": seq, "qos": qos, "ver": ver, "active": active, "reasons": reasons }
    })
    .to_string()
}

pub(crate) fn create_op_17(
    lobby_id: u64,
    self_mute: bool,
    self_deaf: bool,
    self_video: Option<bool>,
    preferred_region: Option<&str>,
) -> String {
    use serde_json::Map;

    let mut d = Map::new();
    d.insert("lobby_id".to_string(), Value::from(lobby_id.to_string()));
    d.insert("self_mute".to_string(), json!(self_mute));
    d.insert("self_deaf".to_string(), json!(self_deaf));
    if let Some(v) = self_video {
        d.insert("self_video".to_string(), json!(v));
    }
    if let Some(r) = preferred_region {
        d.insert("preferred_region".to_string(), json!(r));
    }

    json!({ "op": 17, "d": d }).to_string()
}

pub(crate) fn create_op_42(lobby_id: u64) -> String {
    json!({ "op": 42, "d": { "lobby_id": lobby_id.to_string() } }).to_string()
}

pub(crate) fn create_op_28(payload: Value) -> String {
    json!({ "op": 28, "d": payload }).to_string()
}

pub(crate) fn create_op_30(
    guild_id: u64,
    channel_ids_hash: Option<&str>,
    role_ids_hash: Option<&str>,
    emoji_ids_hash: Option<&str>,
    sticker_ids_hash: Option<&str>,
) -> String {
    use serde_json::Map;

    let mut d = Map::new();
    d.insert("guild_id".to_string(), Value::from(guild_id.to_string()));
    for (k, v) in [
        ("channel_ids_hash", channel_ids_hash),
        ("role_ids_hash", role_ids_hash),
        ("emoji_ids_hash", emoji_ids_hash),
        ("sticker_ids_hash", sticker_ids_hash),
    ] {
        if let Some(h) = v {
            d.insert(k.to_string(), json!(h));
        }
    }

    json!({ "op": 30, "d": d }).to_string()
}

pub(crate) fn create_op_43(guild_id: u64, fields: Vec<String>) -> String {
    json!({
        "op": 43,
        "d": { "guild_id": guild_id.to_string(), "fields": fields }
    })
    .to_string()
}
