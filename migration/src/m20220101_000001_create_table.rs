use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_table(
                Table::create()
                    .table(Words::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Words::Id)
                            .integer()
                            .not_null()
                            .primary_key()
                            .auto_increment(),
                    )
                    .col(ColumnDef::new(Words::Word).text().not_null())
                    .col(ColumnDef::new(Words::Phonetic).text())
                    .col(ColumnDef::new(Words::Definition).text())
                    .col(ColumnDef::new(Words::Translation).text())
                    .col(ColumnDef::new(Words::Pos).text())
                    .col(ColumnDef::new(Words::Collins).text())
                    .col(ColumnDef::new(Words::Oxford).text())
                    .col(ColumnDef::new(Words::Tag).text())
                    .col(ColumnDef::new(Words::Bnc).text())
                    .col(ColumnDef::new(Words::Frq).text())
                    .col(ColumnDef::new(Words::Exchange).text())
                    .col(ColumnDef::new(Words::Detail).text())
                    .col(ColumnDef::new(Words::Audio).text())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Words::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Words {
    Table,
    Id,
    Word,
    Phonetic,
    Definition,
    Translation,
    Pos,
    Collins,
    Oxford,
    Tag,
    Bnc,
    Frq,
    Exchange,
    Detail,
    Audio,
}
