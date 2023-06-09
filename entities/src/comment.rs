//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use super::prelude::*;
use chrono::NaiveDateTime;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
// use sea_orm::Column;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "comment")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(column_name = "user_id")]
    pub user_id: i32,
    #[sea_orm(column_name = "video_id")]
    pub video_id: i32,
    pub text: String,
    pub date: Option<NaiveDateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::video::Entity",
        from = "Column::VideoId",
        to = "super::video::Column::Id"
    )]
    Video,
}

impl Related<super::user::Entity> for Comment {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::video::Entity> for Comment {
    fn to() -> RelationDef {
        Relation::Video.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
