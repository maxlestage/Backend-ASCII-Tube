use async_std::path::Path;
use db::db_connection::db_connection;
use entities::video;
// use queries::coment_service::*;
use queries::user_service::*;
// use queries::video_service::*;
use crate::upload::upload;
use queries::structs::{User, Video};
use queries::video_service::create_video;
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
    let test = req.form_data();
    // println!("{:?}", req.body().unwrap());
    println!("{:?}", test.await.unwrap().fields.get("id_user"));
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    //let video = video::ActiveModel::from_json(json!(video_input)).expect("not valid");
    upload(req, res).await.to_owned();
    // if create_video(db_connect, video).await.is_some() {
    //     res.set_status_code(StatusCode::CREATED);
    // } else {
    //     res.render(Text::Json("Bad Request"));
    //     res.set_status_code(StatusCode::BAD_REQUEST);
    // }
}
