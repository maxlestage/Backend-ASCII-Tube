use std::option;

use chrono::Local;
use entities::prelude::*;
use entities::*;

use sea_orm::ActiveModelTrait;
// use sea_orm::ActiveValue;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DeleteResult;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

pub async fn create_comment(
    db: DatabaseConnection,
    comment_input: comment::ActiveModel,
) -> Option<comment::Model> {
    let mut comment_inputed = comment_input;
    comment_inputed.date = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));
    let comment: comment::Model = comment_inputed.insert(&db).await.expect("Insertion loupé");
    Some(comment)
}
