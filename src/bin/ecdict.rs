use dotenvy::dotenv;
use migration::{Migrator, MigratorTrait};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ConnectionTrait, Database, DatabaseConnection, Statement, TransactionTrait
};

use runslate::db::ecdict::entities::words::ActiveModel as EcdictWord;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    // .env 里的 SQLite 数据库路径
    let db = Database::connect(std::env::var("DATABASE_URL")?).await?;

    // 🔥 从根 crate 运行 migration！
    Migrator::up(&db, None).await?;
    import_csv(&db, "stardict.csv").await?;
    println!("Database migration finished!");

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct CsvEcdictRecord {
    word: String,
    phonetic: Option<String>,
    definition: Option<String>,
    translation: Option<String>,
    pos: Option<String>,
    collins: Option<String>,
    oxford: Option<String>,
    tag: Option<String>,
    bnc: Option<String>,
    frq: Option<String>,
    exchange: Option<String>,
    detail: Option<String>,
    audio: Option<String>,
}

async fn import_csv(db: &DatabaseConnection, csv_path: &str) -> anyhow::Result<()> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(csv_path)?;
    let mut count = 0u32;
    let mut batch: Vec<EcdictWord> = Vec::with_capacity(1000);
    for result in reader.deserialize::<CsvEcdictRecord>() {
        let record = result?;
        let model = EcdictWord {
            word: Set(record.word),
            phonetic: Set(record.phonetic),
            definition: Set(record.definition),
            translation: Set(record.translation),
            pos: Set(record.pos),
            collins: Set(record.collins),
            oxford: Set(record.oxford),
            tag: Set(record.tag),
            bnc: Set(record.bnc),
            frq: Set(record.frq),
            exchange: Set(record.exchange),
            detail: Set(record.detail),
            audio: Set(record.audio),
            ..Default::default()
        };
        batch.push(model);
        if batch.len() >= 1000 {
            let txn = db.begin().await?;
            for m in batch.drain(..) {
                m.insert(&txn).await?;
                count += 1;
            }
            txn.commit().await?;
        }
    }
    if !batch.is_empty() {
        let txn = db.begin().await?;
        for r in batch.drain(..) {
            r.insert(&txn).await?;
            count += 1;
        }
        txn.commit().await?;
    }
    Ok(())
}
