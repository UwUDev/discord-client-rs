use std::error::Error;

pub(crate) mod etf;
pub mod events;
pub mod gateway;
pub(crate) mod utils;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;
