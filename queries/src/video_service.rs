use chrono::Local;
use entities::prelude::Video;
use entities::video;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
// use sea_orm::ActiveValue;

pub async fn create_video(
    db: DatabaseConnection,
    video_input: video::ActiveModel,
) -> Option<video::Model> {
    let mut video_inputed = video_input;
    video_inputed.date = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));
    video_inputed.path_to_json = sea_orm::ActiveValue::Set("not set yet".to_string());
    let video: video::Model = video_inputed.insert(&db).await.expect("Insertion loupé");
    Some(video)
}
pub async fn set_path_to_json(
    db: DatabaseConnection,
    video_input: video::Model,
) -> Option<video::Model> {
    let video_inputed = video_input;
    let video: Option<video::Model> = Video::find_by_id(video_inputed.id)
        .one(&db)
        .await
        .expect("Select loupé");
    let mut video: video::ActiveModel = video.unwrap().into();
    let path = format!(
        "/video/{}/{}",
        video_inputed.user_id.to_owned(),
        video_inputed.id.to_owned()
    );
    video.path_to_json = Set(path);

    let video: video::Model = video.update(&db).await.expect("Insertion loupé");
    Some(video)
}

pub async fn get_video_by_id(db: DatabaseConnection, id: i32) -> Option<video::Model> {
    let video: Option<video::Model> = Video::find_by_id(id).one(&db).await.expect("Select loupé");
    video
}
