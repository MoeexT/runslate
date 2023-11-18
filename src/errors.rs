
#[derive(Debug)]
pub enum Error {
    EnvNotExist(String),
    FileNotExist(String),
    OuterCrateInternalError(String),
    CacheExpired(String),
    CacheNotFound(String),
    SerializeFailed(String),
    DeserializeFailed(String),
    ReadFileError(String),
    OpenFileError(String),
}
