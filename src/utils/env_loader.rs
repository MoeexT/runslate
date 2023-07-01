use std::{env, path::Path};

use log::debug;

use crate::errors::Error;

pub fn load(key: &str) -> Result<String, Error> {
    if let Ok(value) = env::var(key) {
        return Ok(value);
    }
    Err(Error::EnvNotExist(String::from(key)))
}

pub fn load_or_panic(key: &str) -> String {
    let result = env::var(key);
    match result {
        Ok(value) => {
            if value.len() > 0 {
                debug!("[load_or_panic] get value: {} -> {}", key, value);
                return value
            }
            panic!("Empty env value: {}", key)
        },
        Err(err) => panic!("{:#?}", err),
    }
}

pub fn load_or_default(key: &str, default: &str) -> String {
    if let Ok(value) = env::var(key) {
        return if value.len() == 0 {
            debug!("Empty env value, use default: {} -> {}", key, default);
            String::from(default)
        } else {
            value
        };
    }
    debug!("Env not exist, use default: {} -> {}", key, default);
    String::from(default)
}

pub async fn load_env_file(env_path: &str) -> Result<(), Error> {
    let file_path = Path::new(env_path);
    if !file_path.exists() {
        return Err(Error::FileNotExist(String::from(".env file not found.")));
    }

    if let None = dotenv::from_filename(env_path).ok() {
        return Err(Error::OuterCrateInternalError(String::from(
            "[dotenv] Load .env file failed.",
        )));
    }

    Ok(())
}

pub fn clear_empty_env(envs: Vec<&str>) {
    for key in envs {
        if let Ok(value) = load(key) {
            if value.trim().len() == 0 {
                env::remove_var(key);
            }
        }
    }
}
