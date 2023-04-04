use db::db_connection::db_connection;
// use queries::coment_service::*;
use queries::user_service::*;
// use queries::video_service::*;
use crate::structs::User;
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
