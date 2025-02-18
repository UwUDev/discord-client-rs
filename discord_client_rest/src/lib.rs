use std::error::Error;
pub mod rest;
pub mod super_prop;
pub mod rate_limit;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;
