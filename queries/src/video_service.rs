use chrono::Local;
use entities::*;
use sea_orm::ActiveModelTrait;
use sea_orm::DatabaseConnection;

pub async fn upload_video(
    db: DatabaseConnection,
    user_input: video::ActiveModel,
    user: user::ActiveModel,
) -> Option<video::Model> {
    let mut user_inputed = user_input;

    let x = user_inputed.id.clone();
    let y = x.into_wrapped_value().as_ref().to_string();

    user_inputed.date = sea_orm::ActiveValue::Set(Some(Local::now().to_owned().date_naive()));
    user_inputed.path_to_json = sea_orm::ActiveValue::Set("/video/".to_string() + &y);
    user_inputed.user_id = user.id;
    user_inputed.duration = sea_orm::ActiveValue::set(0);

    let video: video::Model = user_inputed.insert(&db).await.expect("Insertion loupé");
    Some(video)
}
