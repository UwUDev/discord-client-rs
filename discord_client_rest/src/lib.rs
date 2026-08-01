use std::error::Error;
pub mod api;
mod bootstrap;
pub mod captcha;
pub mod clearance;
pub mod image;
pub mod mfa;
pub mod rate_limit;
mod response;
pub mod rest;
pub mod sessionless;
pub mod structs;
pub mod super_prop;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;

const MAX_ICON_SIZE: usize = 10 * 1024 * 1024;
