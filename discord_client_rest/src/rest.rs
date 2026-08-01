use crate::api::application::ApplicationRest;
use crate::api::auth::AuthRest;
use crate::api::automod::AutomodRest;
use crate::api::billing::BillingRest;
use crate::api::collectibles::CollectiblesRest;
use crate::api::discovery::DiscoveryRest;
use crate::api::dm::DmRest;
use crate::api::emoji::EmojiRest;
use crate::api::entitlement::EntitlementRest;
use crate::api::family_center::FamilyCenterRest;
use crate::api::group::GroupRest;
use crate::api::guild::GuildRest;
use crate::api::guild_template::GuildTemplateRest;
use crate::api::integration::IntegrationRest;
use crate::api::invite::InviteRest;
use crate::api::message::MessageRest;
use crate::api::notification_center::NotificationCenterRest;
use crate::api::premium_referral::PremiumReferralRest;
use crate::api::presence::PresenceRest;
use crate::api::promotion::PromotionRest;
use crate::api::quests::QuestsRest;
use crate::api::relationship::RelationshipRest;
use crate::api::safety_hub::SafetyHubRest;
use crate::api::scheduled_event::ScheduledEventRest;
use crate::api::self_user::SelfUserRest;
use crate::api::soundboard::SoundboardRest;
use crate::api::stage::StageRest;
use crate::api::sticker::StickerRest;
use crate::api::store::StoreRest;
use crate::api::user::UserRest;
use crate::api::voice::VoiceRest;
use crate::api::webhook::WebhookRest;
use crate::bootstrap::bootstrap_client;
use crate::captcha::{CaptchaRequiredError, SolvedCaptcha};
use crate::mfa::{MfaRequiredError, MfaVerificationRequest};
use crate::rate_limit::{RateLimitError, RateLimiter};
use crate::response::{parse_error_body, rate_limit_from_body};
use crate::structs::context::{Context, ContextHeader};
use crate::structs::referer::{
    DmChannelReferer, GuildChannelReferer, GuildReferer, HomePageReferer, Referer, RefererHeader,
};
use crate::super_prop::build_super_props;
use crate::{BoxedError, BoxedResult};
use current_locale::current_locale;
use derive_builder::Builder;
use discord_client_structs::parser::parse_id_from_token;
use discord_client_structs::structs::application::ApplicationCommandIndex;
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

pub struct RestClient {
    token: String,
    pub user_id: u64,
    client: Client,
    pub api_version: u8,
    pub application_command_index: Option<ApplicationCommandIndex>,
    locale: String,
    timezone: String,
    pub build_numbers: BuildNumbers,
    global_rate_limiter: RateLimiter,
    route_rate_limiters: Arc<Mutex<HashMap<String, RateLimiter>>>,
    client_session: ClientSession,
}

impl RestClient {
    pub async fn connect(
        token: String,
        custom_api_version: Option<u8>,
        custom_build_numbers: Option<BuildNumbers>,
        client_session: Option<ClientSession>,
        proxy: Option<String>,
    ) -> BoxedResult<Self> {
        let user_id = parse_id_from_token(&token).map_err(|_| BoxedError::from("Invalid token"))?;

        let build_numbers = match custom_build_numbers {
            None => find_build_numbers().await?,
            Some(build_num) => build_num,
        };

        let bootstrap = bootstrap_client(custom_api_version, proxy.as_deref()).await?;
        let client = bootstrap.client;
        let api_version = bootstrap.api_version;

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
        let client_session = client_session.unwrap_or_else(|| ClientSession::new());

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
            .header(
                "x-super-properties",
                build_super_props(build_numbers.clone(), client_session.clone()),
            )
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

        let application_command_index = match resp.json::<ApplicationCommandIndex>().await {
            Ok(index) => Some(index),
            Err(e) => {
                error!("Failed to parse application command index: {}", e);
                None
            }
        };

        Ok(Self {
            token,
            user_id,
            client,
            api_version,
            application_command_index,
            locale,
            timezone,
            build_numbers,
            global_rate_limiter: RateLimiter::new(),
            route_rate_limiters: Arc::new(Mutex::new(HashMap::new())),
            client_session,
        })
    }

    pub fn message(&self, channel_id: u64) -> MessageRest<'_> {
        MessageRest {
            channel_id,
            client: self,
        }
    }

    pub fn guild(&self, guild_id: Option<u64>) -> GuildRest<'_> {
        GuildRest {
            guild_id,
            client: self,
        }
    }

    pub fn dm(&self) -> DmRest<'_> {
        DmRest { client: self }
    }

    pub fn group(&self) -> GroupRest<'_> {
        GroupRest { client: self }
    }

    pub fn self_user(&self) -> SelfUserRest<'_> {
        SelfUserRest { client: self }
    }

    pub fn auth(&self) -> AuthRest<'_> {
        AuthRest { client: self }
    }

    pub fn invite(&self) -> InviteRest<'_> {
        InviteRest { client: self }
    }

    pub fn user(&self) -> UserRest<'_> {
        UserRest { client: self }
    }

    pub fn relationship(&self) -> RelationshipRest<'_> {
        RelationshipRest { client: self }
    }

    pub fn emoji(&self, guild_id: u64) -> EmojiRest<'_> {
        EmojiRest {
            guild_id,
            client: self,
        }
    }

    pub fn sticker(&self) -> StickerRest<'_> {
        StickerRest { client: self }
    }

    pub fn voice(&self) -> VoiceRest<'_> {
        VoiceRest { client: self }
    }

    pub fn automod(&self, guild_id: u64) -> AutomodRest<'_> {
        AutomodRest {
            guild_id,
            client: self,
        }
    }

    pub fn scheduled_event(&self, guild_id: u64) -> ScheduledEventRest<'_> {
        ScheduledEventRest {
            guild_id,
            client: self,
        }
    }

    pub fn stage(&self) -> StageRest<'_> {
        StageRest { client: self }
    }

    pub fn soundboard(&self) -> SoundboardRest<'_> {
        SoundboardRest { client: self }
    }

    pub fn webhook(&self) -> WebhookRest<'_> {
        WebhookRest { client: self }
    }

    pub fn integration(&self) -> IntegrationRest<'_> {
        IntegrationRest { client: self }
    }

    pub fn guild_template(&self) -> GuildTemplateRest<'_> {
        GuildTemplateRest { client: self }
    }

    pub fn discovery(&self) -> DiscoveryRest<'_> {
        DiscoveryRest { client: self }
    }

    pub fn application(&self) -> ApplicationRest<'_> {
        ApplicationRest { client: self }
    }

    pub fn billing(&self) -> BillingRest<'_> {
        BillingRest { client: self }
    }

    pub fn entitlement(&self) -> EntitlementRest<'_> {
        EntitlementRest { client: self }
    }

    pub fn store(&self) -> StoreRest<'_> {
        StoreRest { client: self }
    }

    pub fn quests(&self) -> QuestsRest<'_> {
        QuestsRest { client: self }
    }

    pub fn collectibles(&self) -> CollectiblesRest<'_> {
        CollectiblesRest { client: self }
    }

    pub fn promotion(&self) -> PromotionRest<'_> {
        PromotionRest { client: self }
    }

    pub fn notification_center(&self) -> NotificationCenterRest<'_> {
        NotificationCenterRest { client: self }
    }

    pub fn family_center(&self) -> FamilyCenterRest<'_> {
        FamilyCenterRest { client: self }
    }

    pub fn premium_referral(&self) -> PremiumReferralRest<'_> {
        PremiumReferralRest { client: self }
    }

    pub fn safety_hub(&self) -> SafetyHubRest<'_> {
        SafetyHubRest { client: self }
    }

    pub fn presence(&self) -> PresenceRest<'_> {
        PresenceRest { client: self }
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

    pub async fn put<T, B: Clone>(
        &self,
        path: &str,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default + Send,
        B: Serialize + Send + Sync,
    {
        self.request(Method::PUT, path, None, body, req_properties)
            .await
    }

    pub async fn patch<T, B: Clone>(
        &self,
        path: &str,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default + Send,
        B: Serialize + Send + Sync,
    {
        self.request(Method::PATCH, path, None, body, req_properties)
            .await
    }

    pub async fn delete<T, B: Clone>(
        &self,
        path: &str,
        body: Option<B>,
        req_properties: Option<RequestProperties>,
    ) -> BoxedResult<T>
    where
        T: DeserializeOwned + Default + Send,
        B: Serialize + Send + Sync,
    {
        self.request(Method::DELETE, path, None, body, req_properties)
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
            .headers(self.build_headers(req_properties)?);

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
            401 => {
                let bytes = resp.bytes().await?;
                let json = parse_error_body(&bytes, 401, url)?;

                if json["code"].is_i64() {
                    let code = json["code"].as_i64().unwrap();
                    if code == 60003 {
                        let mfa_value = json.get("mfa").unwrap();
                        let mfa_request =
                            serde_json::from_value::<MfaVerificationRequest>(mfa_value.clone())
                                .map_err(|e| Box::new(e) as BoxedError)?;

                        let mfa_error = MfaRequiredError {
                            verification_request: mfa_request,
                        };

                        return Err(Box::new(mfa_error));
                    }
                }

                return Err("Invalid token".into());
            }
            204 => return Ok(T::default()),
            200..=299 => (),
            429 => {
                let bytes = resp.bytes().await?;
                return Err(Box::new(rate_limit_from_body(&bytes, url)));
            }
            400 => {
                let bytes = resp.bytes().await?;
                let resp_json = parse_error_body(&bytes, 400, url)?;

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

    fn build_headers(&self, req_properties: Option<RequestProperties>) -> BoxedResult<HeaderMap> {
        let mut headers = HeaderMap::new();

        headers.insert(
            "Authorization",
            self.token.parse().map_err(|e| Box::new(e) as BoxedError)?,
        );

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

#[derive(Debug, Clone, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct RequestProperties {
    pub referer: Option<Referer>,
    pub context: Option<Context>,
    pub solved_captcha: Option<SolvedCaptcha>,
}

impl RequestProperties {
    pub fn from_referer(referer: Referer) -> Self {
        Self {
            referer: Some(referer),
            ..Default::default()
        }
    }

    pub fn home() -> Self {
        Self::from_referer(HomePageReferer.into())
    }

    pub fn guild(guild_id: u64) -> Self {
        Self::from_referer(GuildReferer { guild_id }.into())
    }

    pub fn guild_channel(guild_id: u64, channel_id: u64) -> Self {
        Self::from_referer(
            GuildChannelReferer {
                guild_id,
                channel_id,
            }
            .into(),
        )
    }

    pub fn dm_channel(channel_id: u64) -> Self {
        Self::from_referer(DmChannelReferer { channel_id }.into())
    }

    pub fn with_context(mut self, context: Context) -> Self {
        self.context = Some(context);
        self
    }

    pub fn with_solved_captcha(mut self, solved_captcha: SolvedCaptcha) -> Self {
        self.solved_captcha = Some(solved_captcha);
        self
    }
}
