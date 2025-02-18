use crate::BoxedResult;
use crate::events::structs::gateway::GatewayPayload;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use rquest::Impersonate::Chrome133;
use rquest::ImpersonateOS::Windows;
use rquest::{Client, Impersonate, Message, WebSocket};
use serde_json::Value;
use std::io::Write;
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::Mutex;
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};

pub struct GatewayClient {
    pub token: String,
    rx: Arc<Mutex<SplitStream<WebSocket>>>,
    tx: Arc<Mutex<SplitSink<WebSocket, Message>>>,
    zlib_decompressor: Arc<Mutex<ZlibStreamDecompressor>>,
    pub heartbeat_interval: u64,
    pub session_id: Option<String>,
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
    capabilities: u32,
    build_number: u32,
}

impl GatewayClient {
    pub async fn connect(token: String, capabilities: u32, build_number: u32) -> BoxedResult<Self> {
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

        let tx_clone = Arc::clone(&tx);
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    heartbeat_interval - 2000,
                ))
                .await;

                if tx_clone
                    .lock()
                    .await
                    .send(Message::Text(r#"{"op":1,"d":null}"#.to_string()))
                    .await
                    .is_err()
                {
                    break;
                }
            }
        });

        Ok(Self {
            token,
            rx: Arc::new(Mutex::new(rx)),
            tx,
            zlib_decompressor: Arc::new(Mutex::new(decompress)),
            heartbeat_interval,
            session_id: None,
            analytics_token: None,
            auth_session_id_hash: None,
            capabilities,
            build_number,
        })
    }

    pub async fn next_event(&mut self) -> BoxedResult<crate::events::Event> {
        let mut rx = self.rx.lock().await;
        let mut decompress = self.zlib_decompressor.lock().await;

        loop {
            let message = rx.try_next().await?;

            if let Some(message) = message {
                match message {
                    Message::Text(text) => {
                        let payload: GatewayPayload = serde_json::from_str(&text).unwrap();
                        return Ok(crate::events::parse_gateway_payload(payload)?);
                    }
                    Message::Binary(bin) => {
                        let vec = match decompress.decompress(bin) {
                            Ok(vec) => vec,
                            Err(ZlibDecompressionError::NeedMoreData) => continue,
                            Err(_err) => return Err("Broken frame".into()),
                        };

                        let text = String::from_utf8(vec).unwrap();

                        std::fs::remove_file("event.json").unwrap_or_default();

                        let mut file = std::fs::OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(false)
                            .open("event.json")
                            .unwrap();

                        file.write_all(text.as_bytes()).unwrap();

                        let payload: GatewayPayload = serde_json::from_str(&text).unwrap();
                        let event = crate::events::parse_gateway_payload(payload)?;

                        if let crate::events::Event::Ready(ready) = event.clone() {
                            self.session_id = Some(ready.session_id);
                            self.analytics_token = Some(ready.analytics_token);
                            self.auth_session_id_hash = Some(ready.auth_session_id_hash);
                        }

                        return Ok(event);
                    }
                    _ => {}
                }
            }
        }
    }

    pub async fn reconnect(&mut self) -> BoxedResult<()> {
        let new_client =
            Self::connect(self.token.clone(), self.capabilities, self.build_number).await?;
        *self = new_client;
        Ok(())
    }

    pub async fn bulk_guild_subscribe(&mut self, guild_ids: Vec<u64>) -> BoxedResult<()> {
        let op_37 = Self::create_op_37(guild_ids);
        self.tx.lock().await.send(Message::Text(op_37)).await?;
        Ok(())
    }

    fn create_op_37(guild_ids: Vec<u64>) -> String {
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
}
