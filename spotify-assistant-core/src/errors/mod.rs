pub mod collections;
pub mod configuration;

use crate::errors::collections::CollectionError;
use crate::errors::configuration::ConfigurationError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyAssistantError {
    #[error("Enum error: {0}")]
    EnumError(#[from] CollectionError),
    #[error("Configuration error: {0}")]
    ConfigurationError(#[from] ConfigurationError),
    #[error("Unknown error occurred")]
    Unknown,
}
