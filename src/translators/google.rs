use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, trace};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{
    dyer::{Colors, Dye},
    env_loader,
};

use super::{Lang, Translator};

const GOOGLE_URL: &str = "https://translate.googleapis.com/translate_a/single";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Google;

#[async_trait]
impl Translator for Google {
    async fn translate(
        &self,
        words: &str,
        source: &Lang,
        target: &Lang,
    ) -> Result<HashMap<String, Value>, Error> {
        trace!("Google: Start to post request.");

        let url = env_loader::load_or_default("RUNSLATE_GOOGLE_URL", GOOGLE_URL);
        let from = match_lang(source);
        let to = match_lang(target);
        let query = vec![
            ("q", words),
            ("sl", &from),
            ("tl", &to),
            ("dt", "t"),
            ("dt", "bd"),
            ("dj", "1"),
            ("client", "gtx"),
        ];

        trace!("Request data generated.");
        debug!("url: {}", url);
        debug!("from: {}, to: {}", from, to);
        debug!("query: {:#?}", query);

        let client = Client::new();
        debug!("reqwest client: {:#?}", client);

        Ok(client
            .get(url)
            .query(&query)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?)
    }

    fn show(&self, response: &HashMap<String, Value>, _more: bool) {
        trace!("Google: parsing response data.");

        // 句子
        if let Some(Value::Array(sentences)) = response.get("sentences") {
            if let Some(Value::Object(sentence)) = sentences.get(0) {
                if let Some(Value::String(trans)) = sentence.get("trans") {
                    println!("{}", trans.dye(Colors::BrightBlack))
                }
            }
        }

        // 词
        if let Some(Value::Array(dicts)) = response.get("dict") {
            for dict in dicts {
                if let Some(Value::String(pos)) = dict.get("pos") {
                    println!("{}", pos.dye(Colors::Blue))
                } else {
                    continue;
                }
                if let Some(Value::Array(entries)) = dict.get("entry") {
                    for (idx, entry) in entries.into_iter().enumerate() {
                        if let Value::Object(entry) = entry {
                            if let Some(Value::String(word)) = entry.get("word") {
                                print!("  {}", word.dye(Colors::BrightBlack));
                            }
                            if let Some(Value::Array(reverse_trans)) =
                                entry.get("reverse_translation")
                            {
                                let mut reverses: Vec<&str> = vec![];
                                for rv in reverse_trans {
                                    if let Value::String(rv) = rv {
                                        reverses.push(rv);
                                    }
                                }
                                println!("{}", reverses.join(", ").dye(Colors::BrightCyan));
                            }
                        }
                        if idx == 2 {
                            break;
                        }
                    }
                }
            }
        }
        trace!("Response parsed.");
    }
}

fn match_lang(lang: &Lang) -> String {
    match lang {
        Lang::Zh => String::from("zh-CN"),
        Lang::Zht => String::from("zh-TW"),
        Lang::Yue => String::from("yue"),
        Lang::Auto => String::from("auto"),
        Lang::En => String::from("en"),
        Lang::Fr => String::from("fr"),
        Lang::De => String::from("de"),
        Lang::It => String::from("it"),
        Lang::Es => String::from("es"),
        Lang::Pt => String::from("Pt"),
        Lang::Ru => String::from("ru"),
        Lang::El => String::from("el"),
        Lang::Ar => String::from("ar"),
        Lang::La => String::from("la"),
        Lang::Ja => String::from("ja"),
        Lang::Ko => String::from("ko"),
    }
}
