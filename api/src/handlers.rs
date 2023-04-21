use crate::upload::upload;
use db::db_connection::db_connection;
use entities::{comment, video};
use queries::coment_service::{delete_comment_by_id, get_comment_by_video_id, insert_comment};
use queries::structs::{Comment, User};
use queries::user_service::*;
use queries::video_service::{
    converter_ascii, create_video, delete_video_by_id, get_video_by_id, set_path_to_json,
};
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

    if select_user_by_id(db_connect.clone(), user_id)
        .await
        .is_some()
    {
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
        upload(file.clone()).await;
        let videocreate: Option<video::Model> = create_video(db_connect.clone(), video).await;
        if videocreate.is_some() {
            let video = set_path_to_json(db_connect, videocreate.unwrap())
                .await
                .expect("Path not created");
            converter_ascii(video.path_to_json, file).await;
            res.set_status_code(StatusCode::CREATED);
        } else {
            res.render(Text::Json("Bad Request"));
            res.set_status_code(StatusCode::BAD_REQUEST);
        }
    } else {
        res.render(Text::Json("User not found"));
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn get_video(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let video = get_video_by_id(db_connect, id).await;
    if video.is_some() {
        res.render(Json(video))
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn delete_video(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let deleted = delete_video_by_id(db_connect, id).await;
    if deleted {
        res.set_status_code(StatusCode::OK);
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn create_comment(req: &mut Request, res: &mut Response) {
    let user_id = req.param::<i32>("user_id").unwrap();
    let video_id = req.param::<i32>("video_id").unwrap();
    let comment_input: Comment = req.extract().await.unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let comment = comment::ActiveModel {
        id: NotSet,
        user_id: sea_orm::ActiveValue::Set(user_id.to_owned()),
        video_id: sea_orm::ActiveValue::Set(video_id.to_owned()),
        text: sea_orm::ActiveValue::Set(comment_input.text),
        date: NotSet,
    };

    let created = insert_comment(db_connect, comment).await;
    if created.is_some() {
        res.set_status_code(StatusCode::OK);
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn get_comment(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("video_id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let comment = get_comment_by_video_id(db_connect, id).await;
    if comment.is_empty() {
        res.set_status_code(StatusCode::NOT_FOUND);
    } else {
        res.render(Json(comment))
    }
}
#[handler]
pub async fn delete_comment(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let deleted = delete_comment_by_id(db_connect, id).await;
    if deleted {
        res.set_status_code(StatusCode::OK);
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}
