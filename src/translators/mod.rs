use std::collections::HashMap;

use async_trait::async_trait;
use clap::ValueEnum;
use reqwest::Error;
use serde_json::Value;

pub mod google;
pub mod youdao;

#[async_trait]
pub trait Translator {
    async fn translate(words: &str, source: &Lang, target: &Lang) -> Result<HashMap<String, Value>, Error>;
    fn show(response: HashMap<String, Value>, more: bool);
}


#[derive(Clone, Debug, ValueEnum)]
pub enum Lang {
    Zh,  // 简体中文
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
