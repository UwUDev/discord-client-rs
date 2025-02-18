use std::error::Error;
mod api;
mod clearance;
pub mod rate_limit;
pub mod rest;
pub mod super_prop;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;
