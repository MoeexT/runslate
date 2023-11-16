use log::{error, warn};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    errors::Error,
    utils::file::{remove_file, write_string},
};

use self::util::{pack, unpack};

pub mod cmd;
pub(self) mod util;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct CacheRecord {
    data: String,
    created_at: u64,
}

pub(crate) fn set<T>(key: &str, value: T) -> bool
where
    T: Sized + Serialize,
{
    let Ok(record) = pack(value) else {
        return false;
    };

    match serde_json::to_string(&record) {
        Ok(content) => write_string(key, content),
        Err(e) => {
            error!("Serialize key ({}) failed: {:?}", key, e);
            false
        }
    }
}

pub(crate) fn get<'a, T>(key: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    let record = match unpack::<&str>(&key) {
        Ok(record) => record,
        Err(_) => {
            let msg = format!("Cache expired or not found");
            warn!("{msg}");
            remove_file(&key);
            return Err(Error::CacheNotFound("".to_string()));
        }
    };

    match serde_json::from_str::<T>(&record.data) {
        Ok(value) => return Ok(value),
        Err(e) => {
            let msg = format!("Deserialize value failed: {e}");
            error!("{msg}");
            return Err(Error::DeserializeFailed(msg));
        }
    }
}
