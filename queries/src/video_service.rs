use crate::structs::Video;
use chrono::Local;
use entities::prelude::*;
use entities::video;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;

pub async fn create_video(
    db: DatabaseConnection,
    video_input: video::ActiveModel,
) -> Option<video::Model> {
    let mut video_inputed = video_input;
    video_inputed.date = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));
    let temp = format!("videos/{}", video_inputed.titre.as_ref().to_owned());
    video_inputed.path_to_json = sea_orm::ActiveValue::Set(temp);

    let video: video::Model = video_inputed.insert(&db).await.expect("Insertion loupé");
    Some(video)
}
