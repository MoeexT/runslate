
#[derive(Debug)]
pub enum Error {
    EnvNotExist(String),
    FileNotExist(String),
    OuterCrateInternalError(String),
}
