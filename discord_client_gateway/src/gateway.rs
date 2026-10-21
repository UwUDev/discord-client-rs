use crate::events::structs::gateway::GatewayPayload;
use crate::utils::*;
use crate::{BoxedError, BoxedResult};
use discord_client_structs::parser::parse_id_from_token;
use discord_client_structs::structs::user::activity::Activity;
use discord_client_structs::structs::user::status::StatusType;
use discord_client_structs::structs::user::status::StatusType::Unknown;
use discord_client_utils::{
    CHROME_VERSION, DESKTOP_NATIVE_BUILD_NUMBER, DESKTOP_OS_SDK_VERSION, DESKTOP_OS_VERSION,
    ELECTRON_VERSION, USER_AGENT, chrome_client_builder, desktop_client_builder,
    desktop_client_version, desktop_user_agent, find_build_numbers,
};
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;
use wreq::Client;
use wreq::ws::WebSocket;
use wreq::ws::message::Message;
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ClientMode {
    #[default]
    Web,
    Desktop,
}

async fn shared_client(mode: ClientMode) -> &'static Client {
    static WEB: OnceLock<Client> = OnceLock::new();
    static DESKTOP: OnceLock<Client> = OnceLock::new();
    let (cell, builder) = match mode {
        ClientMode::Web => (&WEB, chrome_client_builder()),
        ClientMode::Desktop => (
            &DESKTOP,
            desktop_client_builder(desktop_client_version().await),
        ),
    };
    cell.get_or_init(|| {
        builder
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .zstd(true)
            .build()
            .unwrap()
    })
}

fn gateway_url(base: &str, mode: ClientMode) -> String {
    let encoding = match mode {
        ClientMode::Web => "json",
        ClientMode::Desktop => "etf",
    };
    format!("{base}?encoding={encoding}&v=9&compress=zlib-stream")
}

fn decode_frame(mode: ClientMode, data: Vec<u8>) -> BoxedResult<Vec<u8>> {
    match mode {
        ClientMode::Web => Ok(data),
        ClientMode::Desktop => Ok(serde_json::to_vec(&crate::etf::decode(&data)?)?),
    }
}

fn encode_frame(mode: ClientMode, payload: String) -> Message {
    match mode {
        ClientMode::Web => Message::Text(payload.into()),
        ClientMode::Desktop => {
            let value: Value = serde_json::from_str(&payload).unwrap_or(Value::Null);
            Message::Binary(crate::etf::encode(&value).into())
        }
    }
}

fn identify_properties(
    mode: ClientMode,
    build_number: u32,
    native_build_number: Option<u32>,
    client_version: &str,
) -> String {
    let launch_id = uuid::Uuid::new_v4();
    match mode {
        ClientMode::Web => format!(
            r#"{{"os":"Windows","browser":"Chrome","device":"","device_model_name":"","system_locale":"en-US","has_client_mods":false,"browser_user_agent":"{USER_AGENT}","browser_version":"{CHROME_VERSION}","os_version":"10","referrer":"","referring_domain":"","referrer_current":"","referring_domain_current":"","release_channel":"stable","client_build_number":{build_number},"client_event_source":null,"client_launch_id":"{launch_id}","is_fast_connect":true}}"#
        ),
        ClientMode::Desktop => {
            let desktop_user_agent = desktop_user_agent(client_version);
            let native_build_number = native_build_number.unwrap_or(DESKTOP_NATIVE_BUILD_NUMBER);
            format!(
                r#"{{"os":"Windows","browser":"Discord Client","release_channel":"stable","client_version":"{client_version}","os_version":"{DESKTOP_OS_VERSION}","os_arch":"x64","app_arch":"x64","system_locale":"en-US","has_client_mods":false,"client_launch_id":"{launch_id}","browser_user_agent":"{desktop_user_agent}","browser_version":"{ELECTRON_VERSION}","os_sdk_version":"{DESKTOP_OS_SDK_VERSION}","client_build_number":{build_number},"native_build_number":{native_build_number},"client_event_source":null,"is_fast_connect":true}}"#
            )
        }
    }
}

#[derive(Clone, Debug)]
pub struct GatewayClientBuilder {
    token: String,
    automatic_reconnect: bool,
    capabilities: u32,
    client_build_number: Option<u32>,
    native_build_number: Option<u32>,
    intents: Option<u64>,
    mode: ClientMode,
}

impl GatewayClientBuilder {
    pub fn mode(mut self, mode: ClientMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn web(self) -> Self {
        self.mode(ClientMode::Web)
    }

    pub fn desktop(self) -> Self {
        self.mode(ClientMode::Desktop)
    }

    pub fn automatic_reconnect(mut self, automatic_reconnect: bool) -> Self {
        self.automatic_reconnect = automatic_reconnect;
        self
    }

    pub fn capabilities(mut self, capabilities: u32) -> Self {
        self.capabilities = capabilities;
        self
    }

    pub fn client_build_number(mut self, client_build_number: u32) -> Self {
        self.client_build_number = Some(client_build_number);
        self
    }

    pub fn native_build_number(mut self, native_build_number: u32) -> Self {
        self.native_build_number = Some(native_build_number);
        self
    }

    pub fn intents(mut self, intents: u64) -> Self {
        self.intents = Some(intents);
        self
    }

    pub async fn connect(self) -> BoxedResult<GatewayClient> {
        GatewayClient::connect_with(self).await
    }
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
    native_build_number: Option<u32>,
    last_sequence: Arc<AtomicU32>,
    heartbeat_ack: Arc<AtomicBool>,
    automatic_reconnect: bool,
    heartbeat_handle: Option<tokio::task::JoinHandle<()>>,
    pub status: StatusType,
    pub activities: Vec<Activity>,
    pub idling_millis: u64,
    pub afk: bool,
    mode: ClientMode,
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
        let mut builder = Self::builder(token)
            .automatic_reconnect(automatic_reconnect)
            .capabilities(capabilities);
        builder.client_build_number = client_build_number;
        builder.intents = intents;
        builder.connect().await
    }

    pub fn builder(token: impl Into<String>) -> GatewayClientBuilder {
        GatewayClientBuilder {
            token: token.into(),
            automatic_reconnect: true,
            capabilities: 1767421,
            client_build_number: None,
            native_build_number: None,
            intents: None,
            mode: ClientMode::Web,
        }
    }

    async fn connect_with(builder: GatewayClientBuilder) -> BoxedResult<Self> {
        let GatewayClientBuilder {
            token,
            automatic_reconnect,
            capabilities,
            client_build_number,
            native_build_number,
            intents,
            mode,
        } = builder;
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let build_number = match client_build_number {
            None => find_build_numbers().await?.client_build_number,
            Some(build_num) => build_num,
        };

        let websocket = shared_client(mode)
            .await
            .websocket(gateway_url("wss://gateway.discord.gg/", mode))
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
                        let vec = decode_frame(mode, vec)?;
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

        let client_version = match mode {
            ClientMode::Web => "",
            ClientMode::Desktop => desktop_client_version().await,
        };

        let identify = format!(
            r#"{{"op":2,"d":{{"token":"{}","capabilities":{}{},"properties":{},"client_state":{{"guild_versions":{{}}}}}}}}"#,
            token,
            capabilities,
            intents_field,
            identify_properties(mode, build_number, native_build_number, client_version)
        );

        tx.lock()
            .await
            .send(encode_frame(mode, identify))
            .await
            .map_err(|e| BoxedError::from(format!("Failed to send identify: {}", e)))?;

        let last_sequence = Arc::new(AtomicU32::new(0));
        let heartbeat_ack = Arc::new(AtomicBool::new(true));

        let heartbeat_handle = Self::spawn_heartbeat(
            Arc::clone(&tx),
            Arc::clone(&last_sequence),
            Arc::clone(&heartbeat_ack),
            heartbeat_interval,
            mode,
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
            native_build_number,
            last_sequence,
            heartbeat_ack,
            automatic_reconnect,
            heartbeat_handle: Some(heartbeat_handle),
            status: Unknown,
            activities: Vec::new(),
            idling_millis: 0,
            afk: false,
            mode,
        })
    }

    fn spawn_heartbeat(
        tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
        last_sequence: Arc<AtomicU32>,
        heartbeat_ack: Arc<AtomicBool>,
        heartbeat_interval: u64,
        mode: ClientMode,
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
                    .send(encode_frame(mode, payload.to_string()))
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
        let websocket = shared_client(self.mode)
            .await
            .websocket(gateway_url(
                self.resume_gateway_url
                    .as_ref()
                    .ok_or("wss://gateway.discord.gg/")?,
                self.mode,
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
                        let vec = decode_frame(self.mode, vec)?;
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
            self.mode,
        ));

        let session_id = self.session_id.as_ref().ok_or("No session ID")?;
        let sequence = self.last_sequence.load(Ordering::Relaxed);

        let payload = create_op_6(self.token.as_str(), session_id, sequence);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
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
                    let vec = decode_frame(self.mode, vec)?;
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
        let mut new_client = Self::connect_with(GatewayClientBuilder {
            token: self.token.clone(),
            automatic_reconnect: self.automatic_reconnect,
            capabilities: self.capabilities,
            client_build_number: Some(self.build_number),
            native_build_number: self.native_build_number,
            intents: self.intents,
            mode: self.mode,
        })
        .await?;
        new_client.status = self.status.clone();
        new_client.activities = self.activities.clone();
        new_client.idling_millis = self.idling_millis;
        new_client.afk = self.afk;

        self.teardown_connection().await;
        *self = new_client;
        Ok(())
    }

    pub async fn subscribe_member_list(
        &mut self,
        guild_id: u64,
        channels: &[(u64, Vec<[u32; 2]>)],
    ) -> BoxedResult<()> {
        let payload = create_op_14(guild_id, channels, true, false, true);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn bulk_guild_subscribe(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let payload = create_op_37(guild_ids);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn delete_stream(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(19, stream_key);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn watch_stream(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(20, stream_key);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn ping_stream_server(&mut self, stream_key: &str) -> BoxedResult<()> {
        let payload = create_stream_key_op(21, stream_key);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn set_stream_paused(&mut self, stream_key: &str, paused: bool) -> BoxedResult<()> {
        let payload = create_op_22(stream_key, paused);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    async fn send_text(&mut self, payload: String) -> BoxedResult<()> {
        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn update_lobby_voice_states(
        &mut self,
        lobby_id: u64,
        self_mute: bool,
        self_deaf: bool,
        self_video: Option<bool>,
        preferred_region: Option<&str>,
    ) -> BoxedResult<()> {
        self.send_text(create_op_17(
            lobby_id,
            self_mute,
            self_deaf,
            self_video,
            preferred_region,
        ))
        .await
    }

    pub async fn ping_lobby_voice_server(&mut self, lobby_id: u64) -> BoxedResult<()> {
        self.send_text(create_op_42(lobby_id)).await
    }

    pub async fn request_forum_unreads(&mut self, payload: Value) -> BoxedResult<()> {
        self.send_text(create_op_28(payload)).await
    }

    pub async fn get_deleted_entity_ids_not_matching_hash(
        &mut self,
        guild_id: u64,
        channel_ids_hash: Option<&str>,
        role_ids_hash: Option<&str>,
        emoji_ids_hash: Option<&str>,
        sticker_ids_hash: Option<&str>,
    ) -> BoxedResult<()> {
        self.send_text(create_op_30(
            guild_id,
            channel_ids_hash,
            role_ids_hash,
            emoji_ids_hash,
            sticker_ids_hash,
        ))
        .await
    }

    pub async fn request_channel_info(
        &mut self,
        guild_id: u64,
        fields: Vec<String>,
    ) -> BoxedResult<()> {
        self.send_text(create_op_43(guild_id, fields)).await
    }

    pub async fn ping_voice_server(&mut self) -> BoxedResult<()> {
        self.send_text(create_op_5()).await
    }

    pub async fn request_call_connect(&mut self, channel_id: u64) -> BoxedResult<()> {
        self.send_text(create_op_13(channel_id)).await
    }

    pub async fn create_speed_test(&mut self, preferred_region: Option<&str>) -> BoxedResult<()> {
        self.send_text(create_op_32(preferred_region)).await
    }

    pub async fn delete_speed_test(&mut self) -> BoxedResult<()> {
        self.send_text(create_op_33()).await
    }

    pub async fn resync_guild_channels(
        &mut self,
        guild_id: u64,
        obfuscated_channel_ids: Vec<u64>,
    ) -> BoxedResult<()> {
        self.send_text(create_op_38(guild_id, obfuscated_channel_ids))
            .await
    }

    pub async fn request_channel_member_count(
        &mut self,
        guild_id: u64,
        channel_id: u64,
    ) -> BoxedResult<()> {
        self.send_text(create_op_39(guild_id, channel_id)).await
    }

    pub async fn update_time_spent_session_id(
        &mut self,
        initialization_timestamp: u64,
        session_id: &str,
        client_launch_id: &str,
    ) -> BoxedResult<()> {
        self.send_text(create_op_41(
            initialization_timestamp,
            session_id,
            client_launch_id,
        ))
        .await
    }

    pub async fn qos_heartbeat(
        &mut self,
        seq: Option<u32>,
        qos: serde_json::Value,
        ver: u32,
        active: bool,
        reasons: Vec<String>,
    ) -> BoxedResult<()> {
        self.send_text(create_op_40(seq, qos, ver, active, reasons))
            .await
    }

    pub async fn request_channel_statuses(&mut self, guild_id: u64) -> BoxedResult<()> {
        let payload = create_op_36(guild_id);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }

    pub async fn request_soundboard_sounds(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let payload = create_op_31(guild_ids);

        self.tx
            .lock()
            .await
            .send(encode_frame(self.mode, payload))
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
            .send(encode_frame(self.mode, payload))
            .await?;
        Ok(())
    }
}
