use thiserror::Error;

#[derive(Error, Debug)]
pub enum CollectionError {
    #[error("The requested field is not available in the collection object: {0}")]
    FieldNotAvailable(String),
    #[error("Unknown error occurred in collection processing")]
    Unknown,
}