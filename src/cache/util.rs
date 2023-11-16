use std::{time::{SystemTime, UNIX_EPOCH}, path::Path};

use log::{debug, error, info};
use serde::Serialize;

use crate::{
    errors::Error,
    utils::{env_loader, file::read_string},
};

use super::CacheRecord;

/// Serialize value and pack it in [CacheRecord].
pub fn pack<T>(value: T) -> Result<CacheRecord, Error>
where
    T: Sized + Serialize,
{
    let Ok(value) = serde_json::to_string::<T>(&value) else {
        error!("Serialize content failed.");
        return Err(Error::SerializeFailed(String::from("")));
    };

    let cur_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    Ok(CacheRecord {
        data: value,
        created_at: cur_time,
    })
}

/// Read file and deserialize file content to [CacheRecord].
/// If cache expired, an [Error::CacheExpired] will be returned.
pub fn unpack<P: AsRef<Path>>(f_name: P) -> Result<CacheRecord, Error>
{
    match read_string(&f_name) {
        Ok(value) => match serde_json::from_str::<CacheRecord>(&value) {
            Ok(record) => {
                info!("Deserialize file content successfully.");

                let cache_ttl = cache_time();
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                debug!("Cache time: {}", cache_ttl);

                if record.created_at + cache_ttl > now {
                    return Ok(record);
                }

                Err(Error::CacheExpired(f_name.as_ref().to_string_lossy().to_string()))
            }
            Err(e) => Err(Error::DeserializeFailed(e.to_string())),
        },
        Err(e) => Err(e),
    }
}

fn cache_time() -> u64 {
    env_loader::load_or_default("RUNSLATE_CACHE_TIME", "300")
        .parse::<u64>()
        .unwrap_or(300)
}
