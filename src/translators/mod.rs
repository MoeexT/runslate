use std::{collections::HashMap, fmt::Display};

use async_trait::async_trait;
use clap::{Args, ValueEnum};
use log::{error, info, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    cache,
    errors::Error,
    translators::{google::Google, youdao::Youdao},
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

// #[derive(Debug, PartialEq, Serialize, Deserialize)]
// pub struct TranslateRecord {
//     pub data: HashMap<String, Value>,
//     created_at: u64,
// }

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
    let words = args.words.join(" ");
    match args.translator {
        Translators::Google => {
            if !args.no_cache {
                if let Ok(response) = load(
                    &words,
                    &args.source_lang,
                    &args.target_lang,
                    &args.translator,
                ) {
                    info!("Load querying result from cache successfully.");
                    Google::show(&response, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Google::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    // debug!("{:#?}", &response);
                    Google::show(&response, args.more);
                    if !args.no_cache {
                        save(
                            &words,
                            &args.target_lang,
                            &args.source_lang,
                            &args.translator,
                            response,
                        );
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
        Translators::Youdao => {
            if !args.no_cache {
                if let Ok(response) = load(
                    &words,
                    &args.source_lang,
                    &args.target_lang,
                    &args.translator,
                ) {
                    info!("Load querying result from cache successfully.");
                    Youdao::show(&response, args.more);
                    return;
                }
                warn!("Try load cache failed.")
            }

            match Youdao::translate(&words, &args.source_lang, &args.target_lang).await {
                Ok(response) => {
                    // debug!("{:#?}", &response);
                    Youdao::show(&response, args.more);
                    if !args.no_cache {
                        save(
                            &words,
                            &args.source_lang,
                            &args.target_lang,
                            &args.translator,
                            response,
                        );
                    }
                }
                Err(err) => error!("{:#?}", err),
            }
        }
    }
}

fn save(
    query: &str,
    sl: &Lang,
    tl: &Lang,
    translator: &Translators,
    value: HashMap<String, Value>,
) {
    // let file_name = digest(String::from(query) + &sl.to_string() + &tl.to_string() + &translator.to_string());
    let file_name = file_name(query, sl, tl, translator);
    cache::set(&file_name, value);
}

fn load(
    query: &str,
    sl: &Lang,
    tl: &Lang,
    translator: &Translators,
) -> Result<HashMap<String, Value>, Error> {
    let file_name = file_name(query, sl, tl, translator);
    cache::get::<HashMap<String, Value>>(file_name)
}

fn file_name(query: &str, sl: &Lang, tl: &Lang, translator: &Translators) -> String {
    let invalid_path_chars = Regex::new("[/\\\\?%*:|\"<>,;= ]").unwrap();
    let multi_stub = Regex::new("-{2,}").unwrap();

    let sentence = query.trim();
    let sentence = invalid_path_chars.replace_all(sentence, "-");
    let sentence = multi_stub.replace_all(&sentence, "-");
    format!("{sl}-{translator}-{tl}_{sentence}")
}

mod test {
    #[cfg(test)]
    use crate::translators::{file_name, Lang, Translators};

    #[test]
    fn test_file_name() {
        println!(
            "{}",
            file_name(
                " query hello /\\?%*:|\"<>,;= world",
                &Lang::Auto,
                &Lang::Ar,
                &Translators::Google
            )
        )
    }
}
