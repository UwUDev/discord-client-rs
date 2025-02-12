use std::error::Error;

pub mod events;
pub mod gateway;

type BoxedError = Box<dyn Error + Send + Sync>;
type BoxedResult<T> = Result<T, BoxedError>;
