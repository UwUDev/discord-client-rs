use crate::events::structs::gateway::GatewayPayload;
use crate::utils::*;
use crate::{BoxedError, BoxedResult};
use discord_client_structs::parser::parse_id_from_token;
use discord_client_structs::structs::user::activity::Activity;
use discord_client_structs::structs::user::status::StatusType;
use discord_client_structs::structs::user::status::StatusType::Unknown;
use discord_client_utils::find_build_numbers;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use wreq::{Client, Message, WebSocket};
use wreq_util::{Emulation, EmulationOS, EmulationOption};
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};

fn shared_client() -> &'static Client {
    static CLIENT: OnceLock<Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        let emu = EmulationOption::builder()
            .emulation(Emulation::Chrome136)
            .emulation_os(EmulationOS::Windows)
            .build();
        Client::builder().emulation(emu).build().unwrap()
    })
}

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
    intents: Option<u64>,
    build_number: u32,
    last_sequence: Arc<AtomicU32>,
    heartbeat_ack: Arc<AtomicBool>,
    automatic_reconnect: bool,
    heartbeat_handle: Option<tokio::task::JoinHandle<()>>,
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
        client_build_number: Option<u32>,
    ) -> BoxedResult<Self> {
        Self::connect_with_intents(
            token,
            automatic_reconnect,
            capabilities,
            client_build_number,
            None,
        )
        .await
    }

    pub async fn connect_with_intents(
        token: String,
        automatic_reconnect: bool,
        capabilities: u32,
        client_build_number: Option<u32>,
        intents: Option<u64>,
    ) -> BoxedResult<Self> {
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let build_number = match client_build_number {
            None => find_build_numbers().await?.client_build_number,
            Some(build_num) => build_num,
        };

        let websocket = shared_client()
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
                        let json: Value = serde_json::from_slice(&vec).map_err(|e| {
                            BoxedError::from(format!("Failed to parse hello payload: {}", e))
                        })?;
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

        let intents_field = match intents {
            Some(intents) => format!(r#","intents":{}"#, intents),
            None => String::new(),
        };

        let identify = format!(
            r#"{{"op":2,"d":{{"token":"{}","capabilities":{}{},"properties":{{"os":"Windows","browser":"Chrome","device":"","system_locale":"en-US","browser_user_agent":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/136.0.0.0 Safari/537.36","browser_version":"136.0.0.0","os_version":"10","referrer":"","referring_domain":"","referrer_current":"","referring_domain_current":"","release_channel":"stable","client_build_number":{},"client_event_source":null,"design_id":0}},"presence":{{"status":"unknown","since":0,"activities":[],"afk":false}},"compress":false,"client_state":{{"guild_versions":{{}}}}}}}}"#,
            token, capabilities, intents_field, build_number
        );

        tx.lock()
            .await
            .send(Message::Text(identify.into()))
            .await
            .map_err(|e| BoxedError::from(format!("Failed to send identify: {}", e)))?;

        let last_sequence = Arc::new(AtomicU32::new(0));
        let heartbeat_ack = Arc::new(AtomicBool::new(true));

        let heartbeat_handle = Self::spawn_heartbeat(
            Arc::clone(&tx),
            Arc::clone(&last_sequence),
            Arc::clone(&heartbeat_ack),
            heartbeat_interval,
        );

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
            intents,
            build_number,
            last_sequence,
            heartbeat_ack,
            automatic_reconnect,
            heartbeat_handle: Some(heartbeat_handle),
            status: Unknown,
            activities: Vec::new(),
            idling_millis: 0,
            afk: false,
        })
    }

    fn spawn_heartbeat(
        tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
        last_sequence: Arc<AtomicU32>,
        heartbeat_ack: Arc<AtomicBool>,
        heartbeat_interval: u64,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let jitter = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.subsec_nanos() as u64 % 1000)
                .unwrap_or(0);

            let steady = heartbeat_interval.saturating_sub(2000).max(1000);
            let mut delay = (heartbeat_interval * jitter / 1000).clamp(1000, steady);

            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(delay)).await;
                delay = steady;

                if !heartbeat_ack.swap(false, Ordering::AcqRel) {
                    let _ = tx.lock().await.close().await;
                    break;
                }

                let d = last_sequence.load(Ordering::Relaxed);

                let payload = json!({
                    "op": 1,
                    "d": d
                });

                if tx
                    .lock()
                    .await
                    .send(Message::Text(payload.to_string().into()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        })
    }

    async fn teardown_connection(&mut self) {
        if let Some(handle) = self.heartbeat_handle.take() {
            handle.abort();
        }
        let _ = self.tx.lock().await.close().await;
    }

    pub async fn resume(&mut self) -> BoxedResult<()> {
        let websocket = shared_client()
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
                        let json: Value = serde_json::from_slice(&vec).map_err(|e| {
                            BoxedError::from(format!("Failed to parse hello payload: {}", e))
                        })?;
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

        self.teardown_connection().await;

        self.tx = tx;
        self.rx = Arc::new(Mutex::new(rx));
        self.zlib_decompressor = Arc::new(Mutex::new(decompress));
        self.heartbeat_interval = heartbeat_interval;
        self.heartbeat_ack = Arc::new(AtomicBool::new(true));
        self.heartbeat_handle = Some(Self::spawn_heartbeat(
            Arc::clone(&self.tx),
            Arc::clone(&self.last_sequence),
            Arc::clone(&self.heartbeat_ack),
            heartbeat_interval,
        ));

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

    async fn resume_or_reconnect(&mut self) -> BoxedResult<()> {
        if self.session_id.is_some()
            && self.resume_gateway_url.is_some()
            && self.resume().await.is_ok()
        {
            return Ok(());
        }
        self.reconnect().await
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
                        self.resume_or_reconnect().await?;
                        continue;
                    } else {
                        return Err("Connection closed".into());
                    }
                }
            };

            let message = message?;

            match message {
                Message::Text(text) => {
                    let payload: GatewayPayload = serde_json::from_str(&text).map_err(|e| {
                        BoxedError::from(format!("Failed to deserialize payload: {}", e))
                    })?;
                    return Ok(crate::events::parse_gateway_payload(payload)?);
                }
                Message::Binary(bin) => {
                    let mut decompress = self.zlib_decompressor.lock().await;

                    let vec = match decompress.decompress(bin) {
                        Ok(vec) => vec,
                        Err(ZlibDecompressionError::NeedMoreData) => continue,
                        Err(_err) => {
                            *decompress = ZlibStreamDecompressor::new();
                            drop(decompress);
                            if self.automatic_reconnect {
                                self.reconnect().await?;
                                continue;
                            }
                            return Err("Broken frame".into());
                        }
                    };
                    let text = String::from_utf8(vec)
                        .map_err(|e| BoxedError::from(format!("Invalid utf8 payload: {}", e)))?;

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

                    #[cfg(feature = "debug_events")]
                    if let crate::events::Event::ParseError(ref e) = event {
                        match e.dump_to("failed_events") {
                            Ok(path) => eprintln!(
                                "[debug_events] {} failed to parse ({} at '{}') -> {}",
                                e.event_type,
                                e.error,
                                e.path,
                                path.display()
                            ),
                            Err(io) => eprintln!(
                                "[debug_events] {} failed to parse and could not be dumped: {}",
                                e.event_type, io
                            ),
                        }
                    }

                    match &event {
                        crate::events::Event::Ready(ready) => {
                            self.session_id = Some(ready.session_id.clone());
                            self.analytics_token = Some(ready.analytics_token.clone());
                            self.auth_session_id_hash = Some(ready.auth_session_id_hash.clone());
                            self.resume_gateway_url = Some(ready.resume_gateway_url.clone());
                            self.heartbeat_ack.store(true, Ordering::Release);
                        }
                        crate::events::Event::HeartbeatAck(_) => {
                            self.heartbeat_ack.store(true, Ordering::Release);
                        }
                        crate::events::Event::AuthSessionChange(session_change) => {
                            self.auth_session_id_hash =
                                Some(session_change.auth_session_id_hash.clone());
                        }
                        crate::events::Event::InvalidSession(invalid) => {
                            if self.automatic_reconnect {
                                let resumable = invalid.resumable;
                                drop(decompress);
                                if resumable {
                                    self.resume().await?;
                                } else {
                                    self.reconnect().await?;
                                }
                            }
                        }
                        crate::events::Event::GatewayReconnect(_) => {
                            if self.automatic_reconnect {
                                drop(decompress);
                                self.resume_or_reconnect().await?;
                            }
                        }
                        _ => {}
                    }

                    return Ok(event);
                }
                Message::Close(frame) => {
                    if self.automatic_reconnect {
                        self.resume_or_reconnect().await?;
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
        if let Some(handle) = self.heartbeat_handle.take() {
            handle.abort();
        }
        let mut tx = self.tx.lock().await;
        tx.send(Message::Close(None)).await?;
        tx.close().await?;
        Ok(())
    }

    pub async fn reconnect(&mut self) -> BoxedResult<()> {
        let mut new_client = Self::connect_with_intents(
            self.token.clone(),
            self.automatic_reconnect,
            self.capabilities,
            Some(self.build_number),
            self.intents,
        )
        .await?;
        new_client.status = self.status.clone();
        new_client.activities = self.activities.clone();
        new_client.idling_millis = self.idling_millis;
        new_client.afk = self.afk;

        self.teardown_connection().await;
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

    pub async fn update_voice_state(
        &mut self,
        guild_id: Option<u64>,
        channel_id: Option<u64>,
        self_mute: bool,
        self_deaf: bool,
        self_video: Option<bool>,
    ) -> BoxedResult<()> {
        let payload = create_op_4(guild_id, channel_id, self_mute, self_deaf, self_video);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn create_stream(
        &mut self,
        stream_type: &str,
        guild_id: Option<u64>,
        channel_id: u64,
        preferred_region: Option<&str>,
    ) -> BoxedResult<()> {
        let payload = create_op_18(stream_type, guild_id, channel_id, preferred_region);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn delete_stream(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(19, stream_key);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn watch_stream(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(20, stream_key);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn ping_stream_server(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(21, stream_key);

        self.tx
            .lock()
            .await
            .send(Message::Text(payload.into()))
            .await?;
        Ok(())
    }

    pub async fn set_stream_paused(&mut self, stream_key: &str, paused: bool) -> BoxedResult<()> {
        let payload = create_op_22(stream_key, paused);

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
