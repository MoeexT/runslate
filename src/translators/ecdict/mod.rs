use sea_orm::{ColumnTrait, Database, DatabaseConnection};
use serde_json::Value;

use crate::{
    errors::Error,
    translators::{Lang, Translator},
    utils::dyer::{Colors, Dye},
};

pub mod entities;

pub struct Ecdict;

#[async_trait::async_trait]
impl Translator for Ecdict {
    async fn translate(&self, words: &str, _source: &Lang, _target: &Lang) -> Result<Value, Error> {
        use sea_orm::EntityTrait;
        use sea_orm::QueryFilter;
        let db = connect_db().await;
        let word = words.split(' ').next().unwrap_or("");
        let result = entities::words::Entity::find()
            .filter(entities::words::Column::Word.eq(word))
            .one(&db)
            .await
            .unwrap();
        Ok(serde_json::to_value(result)?)
    }
    fn show(&self, _response: &Value, _more: bool) {
        if let Some(Value::String(word)) = _response.get("word") {
            println!("{}", word);
        }
        if let Some(Value::String(exchange)) = _response.get("exchange") {
            println!("{}", exchange.replace("/", ", ").dye(Colors::BrightCyan));
        }
        if let Some(Value::String(phonetic)) = _response.get("phonetic") {
            println!("{}", phonetic.dye(Colors::BrightYellow));
        }
        if let Some(Value::String(definition)) = _response.get("definition") {
            println!(
                "{}",
                definition.replace("\\n", "\n").dye(Colors::BrightGreen)
            );
        }
        if let Some(Value::String(translation)) = _response.get("translation") {
            println!("{}", translation.replace("\\n", "\n").dye(Colors::Blue));
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
