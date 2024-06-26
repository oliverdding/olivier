pub use sea_orm_migration::prelude::*;

mod m20240118_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migration_table_name() -> sea_orm::DynIden {
        Alias::new("migrations").into_iden()
    }

    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20240118_000001_create_table::Migration)]
    }
}
