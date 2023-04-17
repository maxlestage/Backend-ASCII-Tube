use crate::upload::upload;
use db::db_connection::db_connection;
use entities::video;
use queries::structs::User;
use queries::user_service::*;
use queries::video_service::{create_video, set_path_to_json};
use salvo::http::StatusCode;
use salvo::{handler, prelude::*};
use sea_orm::{entity::*, DatabaseConnection};
use serde_json::json;

#[handler]
pub async fn hello_world() -> &'static str {
    "Hello there!"
}

#[handler]
pub async fn hello_by_id(req: &mut Request) -> String {
    req.params().get("id").cloned().unwrap_or_default()
}

#[handler]
pub async fn sign_up(user_input: User, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let user = entities::user::ActiveModel::from_json(json!(user_input)).expect("not valid");

    if create_user(db_connect, user).await.is_some() {
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.render(Text::Json("Bad Request"));
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}

#[handler]
pub async fn upload_video(req: &mut Request, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let data = req.form_data();
    let value = data.await.unwrap().clone();

    let file = value.clone().files.get("file");
    let user_id = value
        .clone()
        .fields
        .get("id_user")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let title = value.clone().fields.get("title").unwrap();
    let description = value.clone().fields.get("description").unwrap();

    let video = video::ActiveModel {
        id: NotSet,
        user_id: sea_orm::ActiveValue::Set(user_id.to_owned()),
        title: sea_orm::ActiveValue::Set(title.to_owned()),
        description: sea_orm::ActiveValue::Set(description.to_owned()),
        date: NotSet,
        path_to_json: NotSet,
    };
    upload(file, res).await.to_owned();
    let videocreate: Option<video::Model> = create_video(db_connect.clone(), video).await;
    if videocreate.is_some() {
        set_path_to_json(db_connect, videocreate.unwrap())
            .await
            .expect("Path not created");
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.render(Text::Json("Bad Request"));
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}
