use std::collections::HashMap;

use reqwest::Error;
use sea_orm::{ColumnTrait, Database, DatabaseConnection};
use serde_json::Value;

use crate::{
    translators::{Lang, Translator},
    utils::dyer::{Colors, Dye},
};

pub mod entities;

pub struct Ecdict;

#[async_trait::async_trait]
impl Translator for Ecdict {
    async fn translate(
        &self,
        words: &str,
        _source: &Lang,
        _target: &Lang,
    ) -> Result<HashMap<String, Value>, Error> {
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;
        let db = connect_db().await;
        let word = words.split(' ').next().unwrap_or("");
        let result = entities::words::Entity::find()
            .filter(entities::words::Column::Word.eq(word))
            .one(&db)
            .await
            .unwrap();
        let results: HashMap<String, Value> = match serde_json::to_value(result) {
            Ok(Value::Object(map)) => map.into_iter().collect(),
            _ => HashMap::new(),
        };
        Ok(results)
    }
    fn show(&self, _response: &HashMap<String, Value>, _more: bool) {
        if let Some(word) = _response.get("word") {
            println!("{}", word);
        }
        if let Some(phonetic) = _response.get("phonetic") {
            println!("{}", phonetic.as_str().unwrap().dye(Colors::BrightYellow));
        }
        if let Some(definition) = _response.get("definition") {
            println!("{}", definition.as_str().unwrap().dye(Colors::BrightGreen));
        }
        if let Some(translation) = _response.get("translation") {
            println!("{}", translation.as_str().unwrap().dye(Colors::Blue));
        }
    }
}

pub async fn connect_db() -> DatabaseConnection {
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or("sqlite://dictionary.db?mode=rwc".to_string());
    Database::connect(&database_url).await.unwrap()
}

pub async fn query(db: &DatabaseConnection, word: &str) -> Option<entities::words::Model> {
    use sea_orm::EntityTrait;
    use sea_orm::QueryFilter;

    let result = entities::words::Entity::find()
        .filter(entities::words::Column::Word.eq(word))
        .one(db)
        .await
        .unwrap();
    result
}
