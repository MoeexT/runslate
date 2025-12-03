
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Environment variable does not exist: {0}")]
    EnvNotExist(String),

    #[error("File does not exist: {0}")]
    FileNotExist(String),

    #[error("External crate internal error: {0}")]
    OuterCrateInternalError(String),
    
    #[error("Cache expired: {0}")]
    CacheExpired(String),
    
    #[error("Cache not found: {0}")]
    CacheNotFound(String),
    
    #[error("Serialization failed: {0}")]
    SerializeError(#[from] serde_json::Error),
    
    #[error("Deserialization failed: {0}")]
    DeserializeFailed(String),
    
    #[error("Read file error: {0}")]
    ReadFileError(String),

    #[error("Open file error: {0}")]
    OpenFileError(String),

    #[error("Translate `{0}` failed")]
    TranslateFailed(String),

    #[error("Network error during translation: {0}")]
    TranslateNetworkError(#[from] reqwest::Error),
}
