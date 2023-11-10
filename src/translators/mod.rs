use std::{
    collections::HashMap,
    fmt::Display,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use clap::{Args, ValueEnum};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha256::digest;

use crate::{
    cache::{read_file, remove_file, write_file},
    errors::Error,
    translators::{google::Google, youdao::Youdao},
    utils::env_loader,
};

mod google;
mod youdao;

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

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct QueryArgs {
    /// [enum] Translator
    #[arg(
        short = 't',
        long,
        default_value = "youdao",
        env = "RUNSLATE_TRANSLATOR"
    )]
    pub translator: Translators,

    /// [enum] Source language
    #[arg(
        short = 's',
        long,
        default_value = "auto",
        env = "RUNSLATE_SOURCE_LANG"
    )]
    pub source_lang: Lang,

    /// [enum] Target language
    #[arg(short = 'd', long, default_value = "zh", env = "RUNSLATE_TARGET_LANG")]
    pub target_lang: Lang,

    /// [bool] Print more translation info
    #[arg(short, long, default_value = "true", env = "RUNSLATE_SHOW_MORE")]
    pub more: bool,

    /// [bool] Decides if to use cache
    #[arg(short = 'n', long, default_value = "false", env = "RUNSLATE_NO_CACHE")]
    pub no_cache: bool,

    /// [bool] Print debug details
    #[arg(short = 'v', long, env = "RUNSLATE_VERBOSE")]
    pub verbose: bool,

    /// [strings] Words to translate
    #[arg(num_args=1.., required = true)]
    pub words: Vec<String>,
}

pub async fn translate(args: QueryArgs) {
    let cache_time = env_loader::load_or_default("RUNSLATE_CACHE_TIME", "300");
    let cache_time = cache_time.parse::<u64>().or::<u64>(Ok(300)).unwrap();

    debug!("Cache time: {}", cache_time);

    let words = args.words.join(" ");
    match args.translator {
        Translators::Google => {
            if !args.no_cache {
                if let Ok(response) = load(&words, &args.target_lang, &args.translator, cache_time)
                {
                    info!("Load querying result from cache successfully.");
                    Google::show(&response.data, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Google::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    // debug!("{:#?}", &response);
                    Google::show(&response, args.more);
                    if !args.no_cache {
                        save(&words, &args.target_lang, &args.translator, response);
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
        Translators::Youdao => {
            if !args.no_cache {
                if let Ok(response) = load(&words, &args.target_lang, &args.translator, cache_time)
                {
                    info!("Load querying result from cache successfully.");
                    Youdao::show(&response.data, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Youdao::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    // debug!("{:#?}", &response);
                    Youdao::show(&response, args.more);
                    if !args.no_cache {
                        save(&words, &args.target_lang, &args.translator, response);
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
    }
}

fn save(query: &str, tl: &Lang, translator: &Translators, value: HashMap<String, Value>) {
    let cur_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let record = TranslateRecord {
        created_at: cur_time,
        data: value,
    };
    let file_name = digest(String::from(query) + &tl.to_string() + &translator.to_string());

    match serde_json::to_string(&record) {
        Ok(content) => {
            write_file(file_name, content);
        }
        Err(e) => {
            error!("Serialize query ({}) failed: {:?}", query, e);
        }
    }
}

fn load(
    query: &str,
    tl: &Lang,
    translator: &Translators,
    expire: u64,
) -> Result<TranslateRecord, Error> {
    let file_name = file_name(query, tl, translator);
    let content = read_file(&file_name)?;

    if let Ok(record) = serde_json::from_str::<TranslateRecord>(&content) {
        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        if record.created_at + expire > cur_time {
            // debug!("Found available cache: {:#?}", &record);
            return Ok(record);
        }

        let msg = format!("Cache expired.");
        warn!("{}", &msg);
        remove_file(&file_name);

        return Err(Error::CacheNotFound(msg));
    }

    let msg = format!("Deserialize cache failed.");
    error!("{}", &msg);

    return Err(Error::DeserializeFailed(msg));
}

fn file_name(query: &str, tl: &Lang, translator: &Translators) -> String {
    digest(String::from(query) + &tl.to_string() + &translator.to_string())
}
