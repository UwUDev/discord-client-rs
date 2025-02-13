use crate::BoxedResult;
use crate::events::structs::GatewayPayload;
use futures_util::stream::SplitStream;
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use rquest::Impersonate::Chrome131;
use rquest::ImpersonateOS::Windows;
use rquest::{Client, Impersonate, Message, WebSocket};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use zlib_stream::{ZlibDecompressionError, ZlibStreamDecompressor};

pub struct GatewayClient {
    pub token: String,
    rx: Arc<Mutex<SplitStream<WebSocket>>>,
    zlib_decompressor: Arc<Mutex<ZlibStreamDecompressor>>,
    pub heartbeat_interval: u64,
    pub session_id: Option<String>,
    pub analytics_token: Option<String>,
    pub auth_session_id_hash: Option<String>,
}

impl GatewayClient {
    pub async fn connect(token: String) -> BoxedResult<Self> {
        let imp = Impersonate::builder()
            .impersonate_os(Windows)
            .impersonate(Chrome131)
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
                // todo: build_number
                r#"{{"op":2,"d":{{"token":"{}","capabilities":53608447,"properties":{{"os":"Windows","browser":"Chrome","device":"","system_locale":"en-US","browser_user_agent":"Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36","browser_version":"131.0.0.0","os_version":"10","referrer":"","referring_domain":"","referrer_current":"","referring_domain_current":"","release_channel":"stable","client_build_number":307392,"client_event_source":null,"design_id":0}},"presence":{{"status":"unknown","since":0,"activities":[],"afk":false}},"compress":false,"client_state":{{"guild_versions":{{}}}}}}}}"#,
                token_var
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
            zlib_decompressor: Arc::new(Mutex::new(decompress)),
            heartbeat_interval,
            session_id: None,
            analytics_token: None,
            auth_session_id_hash: None,
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
}
