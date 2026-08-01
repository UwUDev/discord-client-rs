use crate::api::sessionless::experiments::SessionlessExperimentsRest;
use crate::api::sessionless::invite::SessionlessInviteRest;
use crate::bootstrap::{DEFAULT_API_VERSION, build_emulated_client, solve_cloudflare_clearance};
use crate::captcha::CaptchaRequiredError;
use crate::rate_limit::{RateLimitError, RateLimiter};
use crate::response::{parse_error_body, rate_limit_from_body};
use crate::rest::RequestProperties;
use crate::structs::context::ContextHeader;
use crate::structs::referer::RefererHeader;
use crate::super_prop::build_super_props;
use crate::{BoxedError, BoxedResult};
use current_locale::current_locale;
use discord_client_structs::structs::client::{BuildNumbers, ClientSession};
use discord_client_utils::find_build_numbers;
use iana_time_zone::get_timezone;
use log::{error, warn};
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use wreq::header::HeaderMap;
use wreq::{Client, Method, Response};

const API_BASE: &str = "https://discord.com/api/";

pub struct SessionlessClient {
    client: Client,
    pub api_version: u8,
    locale: String,
    timezone: String,
    pub build_numbers: BuildNumbers,
    global_rate_limiter: RateLimiter,
    route_rate_limiters: Arc<Mutex<HashMap<String, RateLimiter>>>,
    client_session: ClientSession,
    fingerprint: Mutex<Option<String>>,
}

impl SessionlessClient {
    pub async fn connect(
        custom_api_version: Option<u8>,
        custom_build_numbers: Option<BuildNumbers>,
        client_session: Option<ClientSession>,
        fingerprint: Option<String>,
        proxy: Option<String>,
        auto_fingerprint: bool,
    ) -> BoxedResult<Self> {
        let build_numbers = match custom_build_numbers {
            None => find_build_numbers().await?,
            Some(build_num) => build_num,
        };

        let http_client = build_emulated_client(proxy.as_deref())?;

        let timezone = get_timezone().unwrap_or("America/New_York".to_string());
        let locale = current_locale().unwrap_or("en-US".to_string());
        let client_session = client_session.unwrap_or_else(ClientSession::new);

        let client = Self {
            client: http_client,
            api_version: custom_api_version.unwrap_or(DEFAULT_API_VERSION),
            locale,
            timezone,
            build_numbers,
            global_rate_limiter: RateLimiter::new(),
            route_rate_limiters: Arc::new(Mutex::new(HashMap::new())),
            client_session,
            fingerprint: Mutex::new(fingerprint),
        };

        if auto_fingerprint && client.fingerprint().await.is_none() {
            if let Err(e) = client.experiments().get_assignments(false).await {
                warn!(
                    "failed to obtain a fingerprint (continuing without one): {}",
                    e
                );
            }
        }

        Ok(client)
    }

    pub fn experiments(&self) -> SessionlessExperimentsRest<'_> {
        SessionlessExperimentsRest { client: self }
    }

    pub fn invite(&self) -> SessionlessInviteRest<'_> {
        SessionlessInviteRest { client: self }
    }

    pub async fn solve_clearance(&self) -> BoxedResult<()> {
        solve_cloudflare_clearance(&self.client).await
    }

    pub async fn fingerprint(&self) -> Option<String> {
        self.fingerprint.lock().await.clone()
    }

    pub async fn set_fingerprint(&self, fingerprint: Option<String>) {
        *self.fingerprint.lock().await = fingerprint;
    }

    pub async fn get<T: DeserializeOwned + Default + Send>(
        &self,
        path: &str,
        query: Option<HashMap<String, String>>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T> {
        self.request::<T, ()>(Method::GET, path, query, None, req_properties)
            .await
    }

    pub async fn post<T, B: Clone>(
        &self,
        path: &str,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default + Send,
        B: Serialize + Send + Sync,
    {
        self.request(Method::POST, path, None, body, req_properties)
            .await
    }

    async fn request<T, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<HashMap<String, String>>,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default + Send,
        B: Serialize + Send + Sync + Clone,
    {
        loop {
            self.global_rate_limiter.wait_if_needed().await;

            let route_limiter = self.get_route_limiter(path).await;
            route_limiter.wait_if_needed().await;

            let _route_lock = route_limiter.route_mutex.lock().await;

            let result = self
                .make_request(
                    method.clone(),
                    path,
                    query.clone(),
                    body.clone(),
                    req_properties.clone(),
                )
                .await;

            drop(_route_lock);

            match result {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if let Some(rate_limit_error) = e.downcast_ref::<RateLimitError>() {
                        if rate_limit_error.global {
                            self.global_rate_limiter
                                .update(rate_limit_error.retry_after)
                                .await;
                        } else {
                            route_limiter.update(rate_limit_error.retry_after).await;
                        }
                        warn!(
                            "Rate limited [{}]! Retrying after {:.2} seconds",
                            if rate_limit_error.global {
                                "global"
                            } else {
                                path
                            },
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

    async fn get_route_limiter(&self, route: &str) -> RateLimiter {
        let mut limiters = self.route_rate_limiters.lock().await;
        if let Some(limiter) = limiters.get(route) {
            limiter.clone()
        } else {
            let limiter = RateLimiter::new();
            limiters.insert(route.to_string(), limiter.clone());
            limiter
        }
    }

    async fn make_request<T, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<HashMap<String, String>>,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default,
        B: Serialize + Send + Sync,
    {
        let mut full_url = format!("{}v{}/{}", API_BASE, self.api_version, path);
        if let Some(query) = query {
            let query_string = query
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&");
            full_url.push_str(&format!("?{}", query_string));
        }
        let mut request = self
            .client
            .request(method, &full_url)
            .headers(self.build_headers(req_properties).await?);

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
            204 => return Ok(T::default()),
            200..=299 => (),
            429 => {
                let bytes = resp.bytes().await?;
                return Err(Box::new(rate_limit_from_body(&bytes, url)));
            }
            400 => {
                let bytes = resp.bytes().await?;
                let resp_json = parse_error_body(&bytes, status.as_u16(), url)?;

                if resp_json["captcha_sitekey"].is_string() {
                    let captcha = serde_json::from_value::<CaptchaRequiredError>(resp_json)
                        .map_err(|e| Box::new(e) as BoxedError)?;
                    return Err(Box::new(captcha));
                }

                error!("Bad request to {}: {}", url, resp_json.to_string());
                return Err("Bad request".into());
            }
            code => {
                let body = resp.text().await?;
                let msg = format!("Request to {} failed with code {}: {}", url, code, body);
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

    async fn build_headers(
        &self,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "X-Debug-Options",
            "bugReporterEnabled"
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "X-Discord-Locale",
            self.locale.parse().map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "X-Discord-Timezone",
            self.timezone
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        headers.insert(
            "X-Super-Properties",
            build_super_props(self.build_numbers.clone(), self.client_session.clone())
                .parse()
                .map_err(|e| Box::new(e) as BoxedError)?,
        );

        if let Some(fingerprint) = self.fingerprint().await {
            headers.insert(
                "X-Fingerprint",
                fingerprint.parse().map_err(|e| Box::new(e) as BoxedError)?,
            );
        }

        if let Some(req_properties) = req_properties {
            if let Some(referer) = req_properties.referer {
                headers.insert(
                    "Referer",
                    referer
                        .get_header_value()
                        .parse()
                        .map_err(|e| Box::new(e) as BoxedError)?,
                );
            }

            if let Some(context) = req_properties.context {
                headers.insert(
                    "X-Context-Properties",
                    context
                        .get_header_value()
                        .parse()
                        .map_err(|e| Box::new(e) as BoxedError)?,
                );
            }

            if let Some(solved_captcha) = req_properties.solved_captcha {
                solved_captcha.add_headers(&mut headers);
            }
        }

        Ok(headers)
    }

    pub fn get_http_client(&self) -> &Client {
        &self.client
    }
}
