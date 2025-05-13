use crate::events::structs::gateway::GatewayPayload;
use crate::utils::*;
use crate::{BoxedError, BoxedResult};
use discord_client_structs::parser::parse_id_from_token;
use discord_client_structs::structs::user::activity::Activity;
use discord_client_structs::structs::user::status::StatusType;
use discord_client_structs::structs::user::status::StatusType::Unknown;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use rquest::{Client, Message, WebSocket};
use rquest_util::{Emulation, EmulationOS, EmulationOption};
use serde_json::{Value, json};
#[cfg(feature = "debug_events")]
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use tokio::sync::Mutex;
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};

pub struct GatewayClient {
    token: String,
    pub user_id: u64,
    rx: Arc<Mutex<SplitStream<WebSocket>>>,
    tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    zlib_decompressor: Arc<Mutex<ZlibStreamDecompressor>>,
    pub heartbeat_interval: u64,
    pub session_id: Option<String>,
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
    resume_gateway_url: Option<String>,
    capabilities: u32,
    build_number: u32,
    last_sequence: Arc<AtomicU32>,
    automatic_reconnect: bool,
    pub status: StatusType,
    pub activities: Vec<Activity>,
    pub idling_millis: u64,
    pub afk: bool,
}

impl GatewayClient {
    pub async fn connect(
        token: String,
        automatic_reconnect: bool,
        capabilities: u32,
        build_number: u32,
    ) -> BoxedResult<Self> {
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let emu = EmulationOption::builder()
            .emulation(Emulation::Chrome134)
            .emulation_os(EmulationOS::Windows)
            .build();

        let client = Client::builder().emulation(emu).build().unwrap();

        let websocket = client
            .websocket("wss://gateway.discord.gg/?encoding=json&v=9&compress=zlib-stream")
            .send()
            .await?
            .into_websocket()
            .await?;

        let (tx, mut rx) = websocket.split();

        let tx = Arc::new(Mutex::new(tx));

        let message = rx.try_next().await?;

        let mut decompress = ZlibStreamDecompressor::new();

        let mut heartbeat_interval = 30_000;
        if let Some(message) = message {
            match message {
                Message::Binary(bin) => match decompress.decompress(bin) {
                    Ok(vec) => {
                        let json: Value = serde_json::from_slice(&vec).unwrap();
                        match json["d"]["heartbeat_interval"].as_u64() {
                            Some(interval) => heartbeat_interval = interval,
                            None => return Err("No heartbeat interval".into()),
                        }
                    }
                    Err(ZlibDecompressionError::NeedMoreData) => {
                        return Err("Need more data".into());
                    }
                    Err(_err) => return Err("Broken frame".into()),
                },
                _ => {}
            }
        }

        let tx_clone = Arc::clone(&tx);
        let token_var = token.clone();
        tokio::spawn(async move {
            let message = format!(
                r#"{{"op":2,"d":{{"token":"{}","capabilities":{},"properties":{{"os":"Windows","browser":"Chrome","device":"","system_locale":"en-US","browser_user_agent":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36","browser_version":"133.0.0.0","os_version":"10","referrer":"","referring_domain":"","referrer_current":"","referring_domain_current":"","release_channel":"stable","client_build_number":{},"client_event_source":null,"design_id":0}},"presence":{{"status":"unknown","since":0,"activities":[],"afk":false}},"compress":false,"client_state":{{"guild_versions":{{}}}}}}}}"#,
                token_var, capabilities, build_number
            );

            tx_clone
                .lock()
                .await
                .send(Message::Text(message.into()))
                .await
                .unwrap();
        });

        let last_sequence = Arc::new(AtomicU32::new(0));

        let tx_clone = Arc::clone(&tx);
        let last_seq_clone = Arc::clone(&last_sequence);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    heartbeat_interval - 2000,
                ))
                .await;

                let d = last_seq_clone.load(Ordering::Relaxed);

                let payload = json!({
                    "op": 1,
                    "d": d
                });

                if tx_clone
                    .lock()
                    .await
                    .send(Message::Text(payload.to_string().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        Ok(Self {
            token,
            user_id,
            rx: Arc::new(Mutex::new(rx)),
            tx,
            zlib_decompressor: Arc::new(Mutex::new(decompress)),
            heartbeat_interval,
            session_id: None,
            analytics_token: None,
            auth_session_id_hash: None,
            resume_gateway_url: None,
            capabilities,
            build_number,
            last_sequence,
            automatic_reconnect,
            status: Unknown,
            activities: Vec::new(),
            idling_millis: 0,
            afk: false,
        })
    }

    pub async fn resume(&mut self) -> BoxedResult<()> {
        let imp = EmulationOption::builder()
            .emulation(Emulation::Chrome134)
            .emulation_os(EmulationOS::Windows)
            .build();

        let client = Client::builder().emulation(imp).build().unwrap();

        let websocket = client
            .websocket(format!(
                "{}?encoding=json&v=9&compress=zlib-stream",
                self.resume_gateway_url
                    .as_ref()
                    .ok_or("wss://gateway.discord.gg/")?
            ))
            .send()
            .await?
            .into_websocket()
            .await?;

        let (tx, mut rx) = websocket.split();

        let tx = Arc::new(Mutex::new(tx));

        let message = rx.try_next().await?;

        let mut decompress = ZlibStreamDecompressor::new();

        let mut heartbeat_interval = 30_000;
        if let Some(message) = message {
            match message {
                Message::Binary(bin) => match decompress.decompress(bin) {
                    Ok(vec) => {
                        let json: Value = serde_json::from_slice(&vec).unwrap();
                        match json["d"]["heartbeat_interval"].as_u64() {
                            Some(interval) => heartbeat_interval = interval,
                            None => return Err("No heartbeat interval".into()),
                        }
                    }
                    Err(ZlibDecompressionError::NeedMoreData) => {
                        return Err("Need more data".into());
                    }
                    Err(_err) => return Err("Broken frame".into()),
                },
                _ => {}
            }
        }

        self.tx = tx;
        self.rx = Arc::new(Mutex::new(rx));
        self.zlib_decompressor = Arc::new(Mutex::new(decompress));
        self.heartbeat_interval = heartbeat_interval;

        let session_id = self.session_id.as_ref().ok_or("No session ID")?;
        let sequence = self.last_sequence.load(Ordering::Relaxed);

        let payload = create_op_6(self.token.as_str(), session_id, sequence);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn next_event(&mut self) -> BoxedResult<crate::events::Event> {
        loop {
            let message = {
                let mut rx_guard = self.rx.lock().await;
                rx_guard.next().await
            };

            let message = match message {
                Some(msg) => msg,
                None => {
                    if self.automatic_reconnect {
                        self.reconnect().await?;
                        continue;
                    } else {
                        return Err("Connection closed".into());
                    }
                }
            };

            let message = message?;

            match message {
                Message::Text(text) => {
                    let payload: GatewayPayload = serde_json::from_str(&text).unwrap();
                    return Ok(crate::events::parse_gateway_payload(payload)?);
                }
                Message::Binary(bin) => {
                    let mut decompress = self.zlib_decompressor.lock().await;

                    let vec = match decompress.decompress(bin) {
                        Ok(vec) => vec,
                        Err(ZlibDecompressionError::NeedMoreData) => continue,
                        Err(_err) => return Err("Broken frame".into()),
                    };
                    let text = String::from_utf8(vec).unwrap();

                    #[cfg(feature = "debug_events")]
                    {
                        std::fs::remove_file("event.json").unwrap_or_default();

                        let mut file = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(false)
                            .open("event.json")
                            .unwrap();

                        file.write_all(text.as_bytes()).unwrap();
                    }

                    let jd = &mut serde_json::Deserializer::from_str(&text);
                    let result: Result<GatewayPayload, _> = serde_path_to_error::deserialize(jd);
                    let payload = match result {
                        Ok(payload) => payload,
                        Err(err) => {
                            return Err(BoxedError::from(format!(
                                "Failed to deserialize payload: {}",
                                err
                            )));
                        }
                    };

                    if let Some(sequence) = payload.s {
                        self.last_sequence.store(sequence, Ordering::Relaxed);
                    }

                    let event = crate::events::parse_gateway_payload(payload)?;

                    if let crate::events::Event::Ready(ready) = event.clone() {
                        self.session_id = Some(ready.session_id);
                        self.analytics_token = Some(ready.analytics_token);
                        self.auth_session_id_hash = Some(ready.auth_session_id_hash);
                        self.resume_gateway_url = Some(ready.resume_gateway_url);
                    } else if self.automatic_reconnect {
                        if let crate::events::Event::InvalidSession(ref invalid) = event {
                            println!("Need to reconnect");
                            if invalid.resumable {
                                drop(decompress);
                                self.resume().await?;
                            } else {
                                drop(decompress);
                                self.reconnect().await?;
                            }
                        }
                    } else if let crate::events::Event::GatewayReconnect(_) = event {
                        if self.automatic_reconnect {
                            drop(decompress);
                            self.reconnect().await?;
                        }
                    }

                    return Ok(event);
                }
                Message::Close(frame) => {
                    if self.automatic_reconnect {
                        self.reconnect().await?;
                        continue;
                    } else {
                        self.tx.lock().await.close().await?;
                        return Err(format!("Closed: {:?}", frame).into());
                    }
                }
                _ => {}
            }
        }
    }

    pub async fn graceful_shutdown(&mut self) -> BoxedResult<()> {
        let mut tx = self.tx.lock().await;
        tx.send(Message::Close(None)).await?;
        tx.close().await?;
        Ok(())
    }

    pub async fn reconnect(&mut self) -> BoxedResult<()> {
        let mut new_client = Self::connect(
            self.token.clone(),
            self.automatic_reconnect,
            self.capabilities,
            self.build_number,
        )
        .await?;
        new_client.status = self.status.clone();
        new_client.activities = self.activities.clone();
        new_client.idling_millis = self.idling_millis;
        new_client.afk = self.afk;
        *self = new_client;
        Ok(())
    }

    pub async fn bulk_guild_subscribe(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let payload = create_op_37(guild_ids);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn request_channel_statuses(&mut self, guild_id: u64) -> BoxedResult<()> {
        let payload = create_op_36(guild_id);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn request_last_messages(
        &mut self,
        guild_id: u64,
        channel_ids: Vec<u64>,
    ) -> BoxedResult<()> {
        let payload = create_op_34(guild_id, channel_ids);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn search_recent_members(
        &mut self,
        guild_id: u64,
        query: &str,
        continuation_token: Option<u64>,
        nonce: Option<&str>,
    ) -> BoxedResult<()> {
        let payload = create_op_35(guild_id, query, continuation_token, nonce);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn request_guild_members(
        &mut self,
        guild_id: u64,
        query: Option<&str>,
        limit: Option<u64>,
        presences: Option<bool>,
        user_ids: Option<Vec<u64>>,
        nonce: Option<&str>,
    ) -> BoxedResult<()> {
        if let Some(user_ids) = &user_ids {
            if user_ids.len() > 100 {
                return Err("User IDs can't be more than 100".into());
            }
        }

        let payload = create_op_8(guild_id, query, limit, presences, user_ids, nonce);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn send_remote_command<T: serde::Serialize>(
        &mut self,
        target_session_id: &str,
        payload: T,
    ) -> BoxedResult<()> {
        let payload = create_op_29(target_session_id, payload);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn request_soundboard_sounds(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let payload = create_op_31(guild_ids);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn update_presence(&mut self) -> BoxedResult<()> {
        let payload = create_op_3(
            self.idling_millis,
            self.activities.clone(),
            self.status,
            self.afk,
        );

        println!("Payload: {}", payload);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }
}
