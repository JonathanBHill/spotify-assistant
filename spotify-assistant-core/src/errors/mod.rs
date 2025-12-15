pub mod configuration;
pub mod enums;

use crate::errors::configuration::ConfigurationError;
use crate::errors::enums::EnumError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyAssistantError {
    #[error("Enum error: {0}")]
    EnumError(#[from] EnumError),
    #[error("Configuration error: {0}")]
    ConfigurationError(#[from] ConfigurationError),
    #[error("Unknown error occurred")]
    Unknown,
}
