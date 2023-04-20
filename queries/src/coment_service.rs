use chrono::Utc;
use entities::prelude::Comment;
use entities::*;
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::QueryOrder;

pub async fn insert_comment(
    db: DatabaseConnection,
    comment_input: comment::ActiveModel,
) -> Option<comment::Model> {
    let mut comment_inputed = comment_input;
    comment_inputed.date = sea_orm::ActiveValue::Set(Some(Utc::now().to_owned().naive_utc()));
    let comment: comment::Model = comment_inputed.insert(&db).await.expect("Insertion loupé");
    Some(comment)
}
pub async fn get_comment_by_video_id(db: DatabaseConnection, id: i32) -> Vec<comment::Model> {
    let comment: Vec<comment::Model> = Comment::find()
        .filter(comment::Column::VideoId.eq(id))
        .order_by_asc(comment::Column::Date)
        .all(&db)
        .await
        .expect("Select loupé");
    comment
}
