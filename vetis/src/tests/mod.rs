use std::error::Error;

mod host;
mod listener;
mod request;
mod response;
mod security;
mod server;

type TestResult<T> = Result<T, Box<dyn Error>>;
