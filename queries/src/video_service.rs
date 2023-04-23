use chrono::Utc;
use entities::prelude::Video;
use entities::video;
use reqwest;
use reqwest::header::ACCEPT;
use reqwest::header::CONTENT_TYPE;
use salvo::http::form::FilePart;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;
use sea_orm::EntityTrait;
use sea_orm::Set;
use std::collections::HashMap;
// use sea_orm::ActiveValue;

pub async fn create_video(
    db: DatabaseConnection,
    video_input: video::ActiveModel,
) -> Option<video::Model> {
    let mut video_inputed = video_input;
    video_inputed.date = sea_orm::ActiveValue::Set(Some(Utc::now().to_owned().naive_utc()));
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
        "/videos/{}/{}",
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

pub async fn delete_video_by_id(db: DatabaseConnection, id: i32) -> bool {
    let video: Option<video::Model> = Video::find_by_id(id).one(&db).await.expect("Select loupé");
    if video.is_some() {
        let video: video::ActiveModel = video.unwrap().into();
        let res = video.delete(&db).await.expect("Can't delete");
        if res.rows_affected == 1 {
            true
        } else {
            false
        }
    } else {
        false
    }
}

pub async fn converter_ascii(path: String, file: Option<&FilePart>) {
    let name = file.unwrap().name().unwrap();
    let mut map = HashMap::new();
    let video_path = "/temp/".to_string() + name;
    let video_output_path = &path;
    map.insert("video_path", video_path);
    map.insert("output_path", video_output_path.to_string());
    let client = reqwest::Client::new();
    let _res = client
        .post("http://video-api:8000/upload")
        .json(&map)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .expect("Error");
}
