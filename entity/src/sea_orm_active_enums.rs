//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "category")]
pub enum Category {
    #[sea_orm(string_value = "ask")]
    Ask,
    #[sea_orm(string_value = "comment")]
    Comment,
    #[sea_orm(string_value = "story")]
    Story,
}
