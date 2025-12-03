use std::env;

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
                debug!("[load_or_panic] got value: {}.", key);
                return value;
            }
            panic!("Empty env value: {}", key)
        }
        Err(err) => panic!("Load env {} panic {:#?}", key, err),
    }
}

pub fn load_or_default(key: &str, default: &str) -> String {
    if let Ok(value) = env::var(key) {
        return if value.len() == 0 {
            debug!("Empty env value, use default: {}.", key);
            String::from(default)
        } else {
            value
        };
    }
    debug!("Env not exist, use default: {}.", key);
    String::from(default)
}

pub fn load_env_file(file_name: &str) -> Result<String, Error> {
    let paths = vec![
        env::current_dir().unwrap().join(file_name),
        home::home_dir().unwrap().join(file_name),
        home::home_dir()
            .unwrap()
            .join(".config/runslate")
            .join(file_name),
        env::current_exe()
            .unwrap()
            .parent()
            .ok_or(Error::FileNotExist("current exe parent".to_string()))?
            .join(file_name),
    ];
    let file_path = paths
        .iter()
        .find(|p| p.exists())
        .ok_or(Error::EnvNotExist(".env".to_string()))?;

    if let None = dotenvy::from_path(&file_path).ok() {
        return Err(Error::OuterCrateInternalError(String::from(
            "[dotenv] Load .env file failed.",
        )));
    }

    Ok(format!("{:?}", file_path))
}

pub fn clear_empty_env(envs: Vec<&str>) {
    for key in envs {
        if let Ok(value) = load(key) {
            if value.trim().len() == 0 {
                unsafe {
                    env::remove_var(key);
                }
            }
        }
    }
}
