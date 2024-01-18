//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.11

use super::sea_orm_active_enums::Category;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "item")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub deleted: bool,
    pub category: Category,
    pub by: i64,
    pub time: DateTime,
    pub text: String,
    pub dead: bool,
    pub parent: i64,
    pub kids: Vec<i64>,
    pub url: String,
    pub score: i32,
    pub title: String,
    pub desendants: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::By",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
