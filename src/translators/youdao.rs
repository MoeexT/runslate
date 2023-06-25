use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use async_trait::async_trait;
use log::{debug, trace};
use reqwest::{header::HeaderMap, Client, Error};
use serde_json::Value;
use sha256::digest;
use uuid::Uuid;

use crate::utils::{
    dyer::{Colors, Dye},
    env_loader,
};

use super::{Lang, Translator};

const YOUDAO_URL: &str = "https://openapi.youdao.com/api";

pub struct Youdao;

#[async_trait]
impl Translator for Youdao {
    async fn translate(
        words: &str,
        source: &Lang,
        target: &Lang,
    ) -> Result<HashMap<String, Value>, Error> {
        trace!("Youdao: Start to post request.");

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded".parse().unwrap(),
        );

        let url = env_loader::load_or_default("RUNSLATE_YOUDAO_URL", YOUDAO_URL);
        let app_id = env_loader::load_or_panic("RUNSLATE_YOUDAO_APP_KEY");
        let app_secret = env_loader::load_or_panic("RUNSLATE_YOUDAO_APP_SECRET");

        let cur_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let input = truncate(words);
        let salt = Uuid::new_v4().to_string();
        let sign = digest(String::from(&app_id) + &input + &salt + &cur_time + &app_secret);
        let from = match_lang(source);
        let to = match_lang(target);

        let mut data = HashMap::new();
        data.insert("q", words);
        data.insert("from", &from);
        data.insert("to", &to);
        data.insert("appKey", &app_id);
        data.insert("salt", &salt);
        data.insert("sign", &sign);
        data.insert("signType", "v3");
        data.insert("curtime", &cur_time);

        trace!("Request data generated.");
        debug!("url: {}", url);
        debug!("from: {}, to: {}", from, to);
        debug!("headers: {:#?}", headers);
        debug!("data: {:#?}", data);

        let client = Client::new();
        debug!("reqwest client: {:#?}", client);

        Ok(client
            .post(url)
            .headers(headers)
            .query(&data)
            .send()
            .await?
            .json::<HashMap<String, Value>>()
            .await?)
    }

    fn show(response: HashMap<String, Value>, more: bool) {
        trace!("Youdao: parsing response data.");

        // 【可选打印项】单词校验后的结果，主要校验字母大小写、单词前含符号、中文简繁体
        if more {
            if let Some(v) = response.get("returnPhrase") {
                if let Value::Array(phrases) = v {
                    let mut original_phrase = String::new();
                    for phrase in phrases {
                        original_phrase.push_str(&phrase.to_string());
                    }
                    println!("{}", original_phrase.dye(Colors::BrightBlack));
                }
            }
        }

        // 翻译结果；查询正确时一定存在
        if let Some(v) = response.get("translation") {
            if let Value::Array(phrases) = v {
                let mut original_phrase = String::new();
                for phrase in phrases {
                    original_phrase.push_str(&phrase.to_string());
                }
                println!("{}", original_phrase.dye(Colors::BrightWhite));
            }
        }

        // 词义；基本词典，查词时才有
        if let Some(Value::Object(v)) = response.get("basic") {
            // 音标
            let mut phonetics = String::new();
            if let Some(Value::String(phonetic_uk)) = v.get("uk-phonetic") {
                phonetics.push_str(&format!("英 [{}]    ", phonetic_uk));
            }
            if let Some(Value::String(phonetic_us)) = v.get("us-phonetic") {
                phonetics.push_str(&format!("美 [{}]    ", phonetic_us));
            }
            if phonetics.len() > 0 {
                println!("{}", phonetics.trim().dye(Colors::BrightYellow));
            }

            // 【可选打印项】
            if more {
                // 词形
                if let Some(Value::Array(wfs)) = v.get("wfs") {
                    let mut wf_vec: Vec<String> = vec![];
                    for wf_wrap in wfs {
                        if let Value::Object(wf) = wf_wrap {
                            for (_, v) in wf {
                                if let Some(Value::String(name)) = v.get("name") {
                                    if let Some(Value::String(value)) = v.get("value") {
                                        wf_vec.push(format!("{}：{}", name, value));
                                    }
                                }
                            }
                        }
                    }
                    println!("{}", wf_vec.join("；").dye(Colors::Cyan));
                }
                // 出现的考试类型
                if let Some(Value::Array(exam_types)) = v.get("exam_type") {
                    let mut types: Vec<&str> = vec![];
                    for t in exam_types {
                        if let Value::String(tp) = t {
                            types.push(tp);
                        }
                    }
                    println!("{}", types.join("; ").dye(Colors::Green));
                }
            }

            // 词义
            if let Some(Value::Array(explains)) = v.get("explains") {
                let mut explain_vec: Vec<&str> = vec![];
                for explain in explains {
                    if let Value::String(explain) = explain {
                        explain_vec.push(explain);
                    }
                }
                println!("{}", explain_vec.join("\n").dye(Colors::BrightYellow))
            }
        }

        // 词义；网络释义，该结果不一定存在
        if let Some(v) = response.get("web") {
            println!("{}", "网络释义".dye(Colors::Blue));
            if let Value::Array(phrases) = v {
                let mut original_phrase = String::new();
                for phrase in phrases {
                    match phrase {
                        Value::Object(kv) => {
                            if let Some(Value::String(key)) = kv.get("key") {
                                original_phrase.push_str(key);
                                original_phrase.push_str(" ");
                            }
                            if let Some(Value::Array(values)) = kv.get("value") {
                                for (idx, value) in values.into_iter().enumerate() {
                                    if let Value::String(v) = value {
                                        original_phrase.push_str(v);
                                        if idx < values.len() - 1 {
                                            original_phrase.push_str("；")
                                        }
                                    }
                                }
                            }
                        }
                        _ => continue,
                    }
                    original_phrase.push('\n');
                }
                println!("{}", original_phrase.dye(Colors::Cyan));
            }
        }

        trace!("Response parsed.");
    }
}

fn truncate(words: &str) -> String {
    let len = words.chars().count();
    if len > 20 {
        let s1 = words.chars().take(10).collect::<String>();
        let s2 = words.chars().skip(len - 10).take(10).collect::<String>();
        return s1 + &s2;
    }
    String::from(words)
}

fn match_lang(lang: &Lang) -> String {
    match lang {
        Lang::Zh => String::from("zh-CHS"),
        Lang::Zht => String::from("zh-CHT"),
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

#[test]
fn test_truncate() {
    dbg!(truncate("1word2word3word4word5word"));
    let cur_time = &SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        .to_string();
    dbg!(cur_time);
}
