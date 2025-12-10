use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("The requested file cannot be found: {0}")]
    FileNotFound(String),
    #[error("Could not parse the requested file: {0}")]
    FileParse(String),
    #[error("Could not deserialize the toml file: {0}")]
    TomlDeserialize(String),
    #[error("Unknown error occurred in collection processing")]
    Unknown,
}
