use crate::api::message::MessageRest;
use crate::clearance::{get_clearance_cookie, get_invisible};
use crate::rate_limit::{RateLimitError, RateLimiter};
use crate::super_prop::build_super_props;
use crate::{BoxedError, BoxedResult};
use current_locale::current_locale;
use discord_client_structs::parser::parse_id_from_token;
use discord_client_structs::structs::application::ApplicationCommandIndex;
use iana_time_zone::get_timezone;
use regex::Regex;
use rquest::Impersonate::Chrome133;
use rquest::ImpersonateOS::Windows;
use rquest::header::HeaderMap;
use rquest::{Client, Impersonate, Method, Response, redirect};
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::time::Duration;

const API_BASE: &str = "https://discord.com/api/";

pub struct RestClient {
    token: String,
    pub user_id: u64,
    client: Client,
    pub api_version: u8,
    pub application_command_index: ApplicationCommandIndex,
    locale: String,
    timezone: String,
    pub build_number: u32,
    rate_limiter: RateLimiter,
}

impl RestClient {
    pub async fn connect(
        token: String,
        custom_api_version: Option<u8>,
        build_number: u32,
    ) -> BoxedResult<Self> {
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let imp = Impersonate::builder()
            .impersonate_os(Windows)
            .impersonate(Chrome133)
            .build();

        let client = Client::builder()
            .impersonate(imp)
            .gzip(true)
            .deflate(true)
            .brotli(true)
            .zstd(true)
            .cookie_store(true)
            .redirect(redirect::Policy::default())
            .build()?;

        // get the cookies and api version
        let resp = client
            .get("https://discord.com/channels/@me")
            .send()
            .await?;

        let body = resp.text().await?;

        let api_version = match custom_api_version {
            Some(v) => v,
            None => {
                let re = Regex::new(r"API_VERSION: (\d+)").unwrap();

                re.captures(&body)
                    .ok_or("Failed to find API version")?
                    .get(1)
                    .ok_or("Failed to find API version")?
                    .as_str()
                    .parse::<u8>()?;

                if let Some(caps) = re.captures(&body) {
                    caps.get(1)
                        .ok_or("Failed to find API version")?
                        .as_str()
                        .parse::<u8>()?
                } else {
                    return Err(Box::from("Failed to find API version"));
                }
            }
        };

        let re = Regex::new(r#"r:'([a-f0-9]+)'"#).unwrap();
        let r: String = re
            .captures(&body)
            .ok_or("Failed to find r")?
            .get(1)
            .ok_or("Failed to find r")?
            .as_str()
            .to_string();

        let re = Regex::new(r#"t:'([a-zA-Z0-9_\-=]+)'"#).unwrap();
        let t: String = re
            .captures(&body)
            .ok_or("Failed to find t")?
            .get(1)
            .ok_or("Failed to find t")?
            .as_str()
            .to_string();

        let (key, s) = get_invisible(&client).await?;

        get_clearance_cookie(&client, r, t, key, s).await?;

        // get experiments cookies
        // todo: parse assignments
        let resp = client
            .get(format!(
                "{}v{}/experiments?with_guild_experiments=true",
                API_BASE, api_version
            ))
            .header("Authorization", token.clone())
            .send()
            .await?;
        let code = resp.status().as_u16();
        if code == 401 {
            return Err(Box::from("Invalid token"));
        }
        if code != 200 {
            return Err(Box::from(format!(
                "Failed to fetch experiments, response code: {}",
                code
            )));
        }
        let _ = resp.text().await?; // ignore the response

        let timezone = get_timezone().unwrap_or("America/New_York".to_string());
        let locale = current_locale().unwrap_or("en-US".to_string());

        // get application command index
        let resp = client
            .get(format!(
                "{}v{}/users/@me/application-command-index",
                API_BASE, api_version
            ))
            .header("Authorization", token.clone())
            .header("x-debug-options", "bugReporterEnabled")
            .header("x-discord-locale", locale.clone())
            .header("x-discord-timezone", timezone.clone())
            .header("x-super-properties", build_super_props(build_number))
            .send()
            .await?;

        let code = resp.status().as_u16();
        if code == 401 {
            return Err(Box::from("Invalid token"));
        }
        if code != 200 {
            return Err(Box::from(format!(
                "Failed to fetch application command index, response code: {}",
                code
            )));
        }

        let application_command_index = resp.json::<ApplicationCommandIndex>().await?;

        // todo: reverse https://discord.com/cdn-cgi/challenge-platform/

        Ok(Self {
            token,
            user_id,
            client,
            api_version,
            application_command_index,
            locale,
            timezone,
            build_number,
            rate_limiter: RateLimiter::new(),
        })
    }

    pub fn message(&self) -> MessageRest {
        MessageRest { client: self }
    }

    pub async fn get<T: DeserializeOwned + Default>(&self, path: &str) -> BoxedResult<T> {
        self.request::<T, ()>(Method::GET, path, None).await
    }

    pub async fn post<T, B: Clone>(&self, path: &str, body: Option<B>) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        self.request(Method::POST, path, body).await
    }

    pub async fn put<T, B: Clone>(&self, path: &str, body: Option<B>) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        self.request(Method::PUT, path, body).await
    }

    pub async fn patch<T, B: Clone>(&self, path: &str, body: Option<B>) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        self.request(Method::PATCH, path, body).await
    }

    pub async fn delete<T, B: Clone>(&self, path: &str, body: Option<B>) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        self.request(Method::DELETE, path, body).await
    }

    async fn request<T, B>(&self, method: Method, path: &str, body: Option<B>) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync + Clone,
    {
        loop {
            self.rate_limiter.wait_if_needed().await;

            let result = self.make_request(method.clone(), path, body.clone()).await;

            match result {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if let Some(rate_limit_error) = e.downcast_ref::<RateLimitError>() {
                        self.rate_limiter
                            .update(rate_limit_error.retry_after, rate_limit_error.global)
                            .await;

                        println!(
                            "Rate limited! Retrying after {:?} seconds",
                            rate_limit_error.retry_after.as_secs_f64()
                        );

                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn make_request<T, B>(
        &self,
        method: Method,
        path: &str,
        body: Option<B>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        let full_url = format!("{}v{}/{}", API_BASE, self.api_version, path);
        let mut request = self
            .client
            .request(method, &full_url)
            .headers(self.build_headers()?);

        if let Some(body_data) = body {
            request = request
                .header("Content-Type", "application/json")
                .json(&body_data);
        }

        let resp = request
            .send()
            .await
            .map_err(|e| Box::new(e) as BoxedError)?;

        self.handle_response(resp, &full_url).await
    }

    async fn handle_response<T: DeserializeOwned + Default>(
        &self,
        resp: Response,
        url: &str,
    ) -> BoxedResult<T> {
        let status = resp.status();
        match status.as_u16() {
            401 => return Err("Invalid token".into()),
            204 => return Ok(T::default()),
            200..=299 => (),
            429 => {
                let json: Value = resp.json().await?;
                let retry_after_secs = json["retry_after"].as_f64().unwrap_or(0.0);
                let retry_after = Duration::from_secs_f64(retry_after_secs);
                let global = json["global"].as_bool().unwrap_or(false);
                return Err(Box::new(RateLimitError::new(retry_after, global)));
            }
            code => {
                let msg = format!("Request to {} failed with code {}", url, code);
                return Err(msg.into());
            }
        }

        let bytes = resp.bytes().await?;
        if bytes.is_empty() {
            Ok(T::default())
        } else {
            serde_json::from_slice(&bytes).map_err(Into::into)
        }
    }

    fn build_headers(&self) -> BoxedResult<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            self.token.parse().map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "x-debug-options",
            "bugReporterEnabled"
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "x-discord-locale",
            self.locale.parse().map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "x-discord-timezone",
            self.timezone
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "x-super-properties",
            build_super_props(self.build_number)
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        Ok(headers)
    }

    // TODO: reties and request queue
}
