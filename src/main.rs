use sea_orm::{Database, DbErr};
use migration::{Migrator, MigratorTrait};


const DATABASE_URL: &str = "postgres://postgres:postgres@localhost:5432";

async fn run() -> Result<(), DbErr> {
    let db: sea_orm::prelude::DatabaseConnection = Database::connect(DATABASE_URL).await?;

    Migrator::up(&db, None).await
}

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    run().await
}
