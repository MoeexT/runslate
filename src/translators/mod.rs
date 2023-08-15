use std::{
    collections::HashMap,
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use clap::ValueEnum;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha256::digest;

use crate::errors::Error;

pub mod google;
pub mod youdao;

const APP_DIR: &str = ".runslate";

#[async_trait]
pub trait Translator {
    async fn translate(
        words: &str,
        source: &Lang,
        target: &Lang,
    ) -> Result<HashMap<String, Value>, reqwest::Error>;
    fn show(response: &HashMap<String, Value>, more: bool);
}

#[derive(Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum Translators {
    Google,
    Youdao,
}

#[derive(Clone, Debug, ValueEnum, Serialize, Deserialize)]
pub enum Lang {
    Zh,   // 简体中文
    Zht,  // 繁体中文
    Yue,  // 粤语
    Auto, // 自动检测
    En,   // 英语, English,
    Fr,   // 法语, French,
    De,   // 德语, German,
    It,   // 意大利语, Italian,
    Es,   // 西班牙语, Spanish,
    Pt,   // 葡萄牙语, Portuguese,
    Ru,   // 俄语, Russian,
    El,   // 希腊语, Greek,
    Ar,   // 阿拉伯语, Arabic,
    La,   // 拉丁语, Latin,
    Ko,   // 韩语, Korean,
    Ja,   // 日语, Japanese,
}

impl Display for Translators {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Translators::Google => write!(f, "google"),
            Translators::Youdao => write!(f, "youdao"),
        }
    }
}

impl Display for Lang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Lang::Zh => write!(f, "Zh"),
            Lang::Zht => write!(f, "Zht"),
            Lang::Yue => write!(f, "Yue"),
            Lang::Auto => write!(f, "Auto"),
            Lang::En => write!(f, "En"),
            Lang::Fr => write!(f, "Fr"),
            Lang::De => write!(f, "De"),
            Lang::It => write!(f, "It"),
            Lang::Es => write!(f, "Es"),
            Lang::Pt => write!(f, "Pt"),
            Lang::Ru => write!(f, "Ru"),
            Lang::El => write!(f, "El"),
            Lang::Ar => write!(f, "Ar"),
            Lang::La => write!(f, "La"),
            Lang::Ko => write!(f, "Ko"),
            Lang::Ja => write!(f, "Ja"),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct TranslateRecord {
    pub data: HashMap<String, Value>,
    created_at: u64,
}

pub fn save_cache(
    query: &str,
    tl: &Lang,
    translator: &Translators,
    value: HashMap<String, Value>,
) -> bool {
    let cur_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let record = TranslateRecord {
        created_at: cur_time,
        data: value,
    };

    match serde_json::to_string(&record) {
        Ok(content) => {
            let file_name = digest(String::from(query) + &tl.to_string() + &translator.to_string());
            let app_dir = home::home_dir().unwrap().join(APP_DIR);

            if !app_dir.exists() {
                fs::create_dir(&app_dir).unwrap();
            }

            let file_path = app_dir.join(file_name);
            match File::create(&file_path) {
                Ok(mut file) => match file.write_all(content.as_bytes()) {
                    Ok(()) => {
                        debug!(
                            "Serialize {} to file {:?} successfully.",
                            query,
                            file_path.display()
                        );
                        return true;
                    }
                    Err(e) => {
                        error!("Write file ({:?}) failed: {:?}", file_path.display(), e);
                        return false;
                    }
                },
                Err(e) => {
                    error!("Open file ({:?}) failed: {:?}", file_path.display(), e);
                    return false;
                }
            }
        }
        Err(e) => {
            error!("Serialize query ({}) failed: {:?}", query, e);
            return false;
        }
    }
}

pub fn load_cache(
    query: &str,
    tl: &Lang,
    translator: &Translators,
    expire: u64,
) -> Result<TranslateRecord, Error> {
    let file_name = digest(String::from(query) + &tl.to_string() + &translator.to_string());
    let app_dir = home::home_dir().unwrap().join(APP_DIR);
    let file_path = app_dir.join(file_name);

    if !file_path.exists() {
        let msg = format!("Cache file doesn't exit: {}.", file_path.display());
        warn!("{}", msg);
        return Err(Error::CacheNotFound(msg));
    }

    if let Ok(mut file) = File::open(&file_path) {
        info!("Reading content from cache file {}", &file_path.display());
        let mut content = String::new();

        if let Ok(_) = file.read_to_string(&mut content) {
            if let Ok(record) = serde_json::from_str::<TranslateRecord>(&content) {
                let cur_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                if record.created_at + expire > cur_time {
                    debug!("Found available cache: {:#?}", &record);
                    return Ok(record);
                }

                let msg = format!("Cache {} has been expired.", file_path.display());
                warn!("{}", &msg);

                let _ = fs::remove_file(&file_path);
                info!("Dropped cache file {}.", file_path.display());

                return Err(Error::CacheNotFound(msg));
            }

            let msg = format!("Deserialize {} failed", file_path.display());
            error!("{}", &msg);

            return Err(Error::DeserializeFailed(msg));
        }

        let msg = format!("Read file ({:?}) failed", file_path.display());
        error!("{}", msg);

        return Err(Error::ReadFileError(msg));
    }

    let msg = format!("Open file ({:?}) failed", file_path.display());
    error!("{}", &msg);

    Err(Error::OpenFileError(msg))
}

#[test]
fn test_save() {
    let app_dir = home::home_dir().unwrap().join(APP_DIR);

    if !app_dir.exists() {
        fs::create_dir(app_dir).unwrap();
    }
}
