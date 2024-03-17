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
    // 1. current directory
    let mut file_path = env::current_dir().unwrap().join(file_name);

    // 2. ~/.config/runslate/
    if !file_path.exists() {
        file_path = home::home_dir()
            .unwrap()
            .join(".config/runslate")
            .join(file_name);
    }

    // 3. current_exe's directory
    if !file_path.exists() {
        file_path = env::current_exe()
            .unwrap()
            .parent()
            .expect("exe file must be in some directory")
            .join(file_name);
    }

    if !file_path.exists() {
        return Err(Error::FileNotExist(String::from(".env file not found.")));
    }

    if let None = dotenv::from_path(&file_path).ok() {
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
                env::remove_var(key);
            }
        }
    }
}
