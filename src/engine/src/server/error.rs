use std::net::SocketAddr;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListenerError {
    #[error("Unable to bind address")]
    BindFailed,
}