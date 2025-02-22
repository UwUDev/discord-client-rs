use rquest::header::HeaderMap;
use serde::Deserialize;
use std::error::Error;
use std::fmt::{Debug, Display};

#[derive(Debug, Deserialize)]
pub struct CaptchaRequiredError {
    pub captcha_key: Vec<String>,
    pub captcha_sitekey: String,
    pub captcha_service: String,
    pub captcha_rqdata: String,
    pub captcha_rqtoken: String,
}
impl Display for CaptchaRequiredError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Captcha required: captcha_key: {:?}, captcha_sitekey: {}, captcha_service: {}, captcha_rqdata: {}, captcha_rqtoken: {}",
            self.captcha_key,
            self.captcha_sitekey,
            self.captcha_service,
            self.captcha_rqdata,
            self.captcha_rqtoken
        )
    }
}

impl Error for CaptchaRequiredError {}

#[derive(Clone, Debug)]
pub struct SolvedCaptcha {
    pub key: String,
    pub rqtoken: String,
}

impl SolvedCaptcha {
    pub fn new(key: String, rqtoken: String) -> Self {
        Self { key, rqtoken }
    }

    pub fn add_headers(&self, headers: &mut HeaderMap) {
        headers.insert("x-captcha-key", self.key.parse().unwrap());
        headers.insert("x-captcha-rqtoken", self.rqtoken.parse().unwrap());
    }
}
