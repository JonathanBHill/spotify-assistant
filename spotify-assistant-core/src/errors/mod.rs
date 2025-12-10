pub mod collections;

use crate::errors::collections::CollectionError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SpotifyAssistantError {
    #[error("Enum error: {0}")]
    EnumError(#[from] CollectionError),
}
