use std::fmt;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::config::ConfigError),
}
