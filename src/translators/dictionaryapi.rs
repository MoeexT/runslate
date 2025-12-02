use std::collections::HashMap;

use async_trait::async_trait;
use log::{debug, trace};
use reqwest::{Client, Error};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::dyer::{Colors, Dye};
use super::{Lang, Translator};

const DICTAPI_URL: &str = "https://api.dictionaryapi.dev/api/v2/entries";


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DictionaryApi;

#[async_trait]
impl Translator for DictionaryApi {
    async fn translate(
        &self,
        words: &str,
        source: &Lang,
        _target: &Lang,
    ) -> Result<HashMap<String, Value>, Error> {
        trace!("DictionaryApi: Start to get request.");

        let lang = match_lang(source);
        let word = words.trim().replace(" ", "%20");
        let url = format!("{}/{}/{}", DICTAPI_URL, lang, word);

        debug!("url: {}", url);
        let client = Client::new();
        debug!("reqwest client: {:#?}", client);

        let v = client.get(&url).send().await?.json::<Value>().await?;

        // we return a map similar to other translators, key "entries"
        let mut map = HashMap::new();
        map.insert(String::from("entries"), v);

        Ok(map)
    }

    fn show(&self, response: &HashMap<String, Value>, _more: bool) {
        trace!("DictionaryApi: parsing response data.");

        if let Some(Value::Array(entries)) = response.get("entries") {
            for entry in entries {
                if let Value::Object(obj) = entry {
                    // word
                    if let Some(Value::String(word)) = obj.get("word") {
                        println!("{}", word.dye(Colors::BrightWhite));
                    }

                    // phonetics
                    if let Some(Value::Array(phonetics)) = obj.get("phonetics") {
                        let mut phonetic_str = String::new();
                        for p in phonetics {
                            if let Value::Object(pobj) = p {
                                if let Some(Value::String(text)) = pobj.get("text") {
                                    phonetic_str.push_str(text);
                                    phonetic_str.push_str(" ");
                                }
                            }
                        }
                        if !phonetic_str.is_empty() {
                            println!("{}", phonetic_str.trim().dye(Colors::BrightYellow));
                        }
                    }

                    // meanings
                    if let Some(Value::Array(meanings)) = obj.get("meanings") {
                        for m in meanings {
                            if let Value::Object(mobj) = m {
                                if let Some(Value::String(pos)) = mobj.get("partOfSpeech") {
                                    println!("{}", pos.dye(Colors::Blue));
                                }
                                if let Some(Value::Array(defs)) = mobj.get("definitions") {
                                    for (i, def) in defs.iter().enumerate() {
                                        if let Value::Object(dobj) = def {
                                            if let Some(Value::String(def_text)) = dobj.get("definition") {
                                                println!("{}. {}", i + 1, def_text.dye(Colors::Cyan));
                                            }
                                            if let Some(Value::String(example)) = dobj.get("example") {
                                                println!("   {}", example.dye(Colors::BrightBlack));
                                            }
                                            if let Some(Value::Array(syns)) = dobj.get("synonyms") {
                                                let syn_vec: Vec<String> = syns.iter().filter_map(|s| {
                                                    if let Value::String(sv) = s { Some(sv.clone()) } else { None }
                                                }).collect();
                                                if !syn_vec.is_empty() {
                                                    println!("   {}", format!("synonyms: {}", syn_vec.join(", ")).dye(Colors::Green));
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // separator between entries
                    println!();
                }
            }
        } else {
            trace!("No entries found in response.");
        }

        trace!("Response parsed.");
    }
}

fn match_lang(lang: &Lang) -> &'static str {
    match lang {
        Lang::En => "en",
        Lang::Es => "es",
        Lang::Fr => "fr",
        Lang::De => "de",
        Lang::It => "it",
        Lang::Pt => "pt-BR",
        Lang::Ru => "ru",
        Lang::Ja => "ja",
        Lang::Ko => "ko",
        Lang::Ar => "ar",
        // fallbacks / Chinese variants not directly supported by dictionaryapi: use English
        Lang::Zh | Lang::Zht | Lang::Yue | Lang::Auto | Lang::El | Lang::La => "en",
    }
}
