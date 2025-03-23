use crate::events::structs::gateway::GatewayPayload;
use crate::utils::*;
use crate::{BoxedError, BoxedResult};
use discord_client_structs::parser::parse_id_from_token;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use rquest::Impersonate::Chrome133;
use rquest::ImpersonateOS::Windows;
use rquest::{Client, CloseCode, Impersonate, Message, WebSocket};
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
    capabilities: u32,
    build_number: u32,
    last_sequence: Arc<AtomicU32>,
    automatic_reconnect: bool,
}

impl GatewayClient {
    pub async fn connect(
        token: String,
        automatic_reconnect: bool,
        capabilities: u32,
        build_number: u32,
    ) -> BoxedResult<Self> {
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let imp = Impersonate::builder()
            .impersonate_os(Windows)
            .impersonate(Chrome133)
            .build();

        let client = Client::builder().impersonate(imp).build().unwrap();

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
                .send(Message::Text(message))
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
                    .send(Message::Text(payload.to_string()))
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
            capabilities,
            build_number,
            last_sequence,
            automatic_reconnect,
        })
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

                    let payload: GatewayPayload = serde_json::from_str(&text).unwrap();
                    if let Some(sequence) = payload.s {
                        self.last_sequence.store(sequence, Ordering::Relaxed);
                    }

                    let event = crate::events::parse_gateway_payload(payload)?;

                    if let crate::events::Event::Ready(ready) = event.clone() {
                        self.session_id = Some(ready.session_id);
                        self.analytics_token = Some(ready.analytics_token);
                        self.auth_session_id_hash = Some(ready.auth_session_id_hash);
                    } else if self.automatic_reconnect {
                        if let crate::events::Event::InvalidSession(ref invalid) = event {
                            println!("Need to reconnect");
                            if invalid.resumable {
                                todo!("Automatic reconnect with resuming");
                            } else {
                                drop(decompress);
                                self.reconnect().await?;
                            }
                        }
                    }

                    return Ok(event);
                }
                Message::Close { code, reason } => {
                    if self.automatic_reconnect {
                        self.reconnect().await?;
                        continue;
                    } else {
                        self.tx.lock().await.close().await?;
                        return Err(format!("Closed: {:?} {:?}", code, reason).into());
                    }
                }
                _ => {}
            }
        }
    }

    pub async fn graceful_shutdown(&mut self) -> BoxedResult<()> {
        let mut tx = self.tx.lock().await;
        tx.send(Message::Close {
            code: CloseCode::Normal,
            reason: None,
        })
        .await?;
        tx.close().await?;
        Ok(())
    }

    pub async fn reconnect(&mut self) -> BoxedResult<()> {
        let new_client = Self::connect(
            self.token.clone(),
            self.automatic_reconnect,
            self.capabilities,
            self.build_number,
        )
        .await?;
        *self = new_client;
        Ok(())
    }

    pub async fn bulk_guild_subscribe(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let payload = create_op_37(guild_ids);

        self.tx.lock().await.send(Message::Text(payload)).await?;
        Ok(())
    }

    pub async fn request_channel_statuses(&mut self, guild_id: u64) -> BoxedResult<()> {
        let payload = create_op_36(guild_id);

        self.tx.lock().await.send(Message::Text(payload)).await?;
        Ok(())
    }

    pub async fn request_last_messages(
        &mut self,
        guild_id: u64,
        channel_ids: Vec<u64>,
    ) -> BoxedResult<()> {
        let payload = create_op_34(guild_id, channel_ids);

        self.tx.lock().await.send(Message::Text(payload)).await?;
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

        self.tx.lock().await.send(Message::Text(payload)).await?;
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

        self.tx.lock().await.send(Message::Text(payload)).await?;
        Ok(())
    }

    pub async fn send_remote_command<T: serde::Serialize>(
        &mut self,
        target_session_id: &str,
        payload: T,
    ) -> BoxedResult<()> {
        let payload = create_op_29(target_session_id, payload);

        self.tx.lock().await.send(Message::Text(payload)).await?;
        Ok(())
    }
}
