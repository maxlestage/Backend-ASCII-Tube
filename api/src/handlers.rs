use crate::upload::upload;
use auth::jwt_auth::JwtClaims;
use base64::engine::general_purpose;
use base64::Engine;
use db::db_connection::db_connection;
use entities::user::UserOmitMP;
use entities::{comment, video};
use queries::coment_service::{delete_comment_by_id, get_comment_by_video_id, insert_comment};
use queries::structs::{Comment, User};
use queries::user_service::*;
use queries::video_service::{
    converter_ascii, create_video, delete_video_by_id, get_video_by_id, get_videos,
    set_path_to_json,
};
use salvo::http::StatusCode;
use salvo::{handler, prelude::*};
use sea_orm::{entity::*, DatabaseConnection};
use serde_json::json;
use std::fs;
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

    if create_user(db_connect.clone(), user.clone())
        .await
        .is_some()
    {
        res.set_status_code(StatusCode::CREATED);
    } else {
        res.render(Text::Json("Bad Request"));
        res.set_status_code(StatusCode::BAD_REQUEST);
    }
}

#[handler]
pub async fn get_user(req: &mut Request, res: &mut Response) {
    let id = req.param::<i32>("id").unwrap();
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let user = select_user_by_id(db_connect, id).await;
    let user_omit_mp = UserOmitMP {
        id: user.clone().unwrap().id,
        firstname: user.clone().unwrap().firstname,
        lastname: user.clone().unwrap().lastname,
        username: user.clone().unwrap().username,
        sign_up_date: user.clone().unwrap().sign_up_date.unwrap(),
        mail: user.clone().unwrap().mail,
    };
    if user.is_some() {
        res.render(Json(user_omit_mp))
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn upload_video(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");

    let data = req.form_data();
    let value = data.await.unwrap().clone();

    let authorize_data = depot.jwt_auth_data::<JwtClaims>().unwrap();

    let file = value.clone().files.get("file");
    let user_id = authorize_data.claims.id;

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
        let video_data = video.unwrap();
        let contents = fs::read_to_string(video_data.clone().path_to_json + "/Style.LIGHT.json")
            .expect("file not found");

        let sound =
            fs::read(video_data.clone().path_to_json + "/sound.mp3").expect("file not found");
        let base64_sound = format!("{}", general_purpose::STANDARD.encode(&sound));

        let video_with_content = video::VideoWithContent {
            id: video_data.id,
            user_id: video_data.user_id,
            title: video_data.title,
            description: video_data.description,
            date: video_data.date,
            path_to_json: video_data.path_to_json,
            content: contents,
            sound: base64_sound,
        };

        res.render(Json(video_with_content));
    } else {
        res.set_status_code(StatusCode::NOT_FOUND);
    }
}

#[handler]
pub async fn get_all_videos(req: &mut Request, res: &mut Response) {
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    let videos = get_videos(db_connect).await;
    res.render(Json(videos));
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
