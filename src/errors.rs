
#[derive(Debug)]
pub enum Error {
    EnvNotExist(String),
    FileNotExist(String),
    OuterCrateInternalError(String),
    CacheNotFound(String),
    SerializeFailed(String),
    DeserializeFailed(String),
    ReadFileError(String),
    OpenFileError(String),
}
