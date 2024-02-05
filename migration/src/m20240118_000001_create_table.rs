use sea_orm_migration::{prelude::*, sea_query::extension::postgres::Type};
use sea_query::ForeignKey;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Name).string().not_null())
                    .col(
                        ColumnDef::new(User::Created)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(User::About)
                            .string()
                            .not_null()
                            .default(Value::String(Some(Box::new("Hello World!".to_string())))),
                    )
                    .col(
                        ColumnDef::new(User::Submitted)
                            .array(ColumnType::BigInteger)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(Category::Table)
                    .values(<Category as sea_orm::Iterable>::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Item::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Item::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Item::Deleted)
                            .boolean()
                            .not_null()
                            .default(Value::Bool(Some(false))),
                    )
                    .col(
                        ColumnDef::new(Item::Category)
                            .enumeration(
                                Category::Table,
                                <Category as sea_orm::Iterable>::iter().skip(1),
                            )
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::By).big_integer().not_null())
                    .col(
                        ColumnDef::new(Item::Time)
                            .timestamp()
                            .not_null()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(ColumnDef::new(Item::Text).string().not_null())
                    .col(ColumnDef::new(Item::Dead).boolean().not_null())
                    .col(ColumnDef::new(Item::Parent).big_integer().not_null())
                    .col(
                        ColumnDef::new(Item::Kids)
                            .array(ColumnType::BigInteger)
                            .not_null(),
                    )
                    .col(ColumnDef::new(Item::Url).string().not_null())
                    .col(ColumnDef::new(Item::Score).integer().not_null())
                    .col(ColumnDef::new(Item::Title).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-item-by")
                    .from(Item::Table, Item::By)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Item::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(Category::Table).to_owned())
            .await?;

        manager
            .drop_foreign_key(ForeignKey::drop().name("fk-item-by").to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Name,
    Created,
    About,
    Submitted,
}

#[derive(Iden, sea_orm::EnumIter)]
pub enum Category {
    Table,
    #[iden = "story"]
    Story,
    #[iden = "comment"]
    Comment,
    #[iden = "ask"]
    Ask,
}

#[derive(DeriveIden)]
enum Item {
    Table,
    Id,
    Deleted,
    Category,
    By,
    Time,
    Text,
    Dead,
    Parent,
    Kids,
    Url,
    Score,
    Title,
}
