use crate::handlers::{
    create_comment, delete_comment, delete_video, get_comment, get_user, get_video, hello_world,
    sign_up, upload_video,
};
use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use db::db_connection::db_connection;
use migration::{Migrator, MigratorTrait};
use salvo::jwt_auth::HeaderFinder;
use salvo::{__private::tracing, prelude::*};
use sea_orm::DatabaseConnection;

pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    Migrator::up(&db_connect, None).await.expect("Error");
    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let router = Router::new()
        .get(hello_world)
        .push(Router::with_path("api/signup").post(sign_up))
        .push(Router::with_path("api/login").post(sign_in))
        .push(Router::with_path("api/video/<id>").get(get_video))
        .push(Router::with_path("api/user/<id>").get(get_user))
        .push(Router::with_path("api/video/<id>").delete(delete_video))
        .push(Router::with_path("api/comment/<video_id>/").get(get_comment))
        .push(
            Router::new().push(
                Router::new()
                    .hoop(auth_handler)
                    .path("api")
                    .push(Router::with_path("video/upload").post(upload_video))
                    .push(Router::with_path("comment/<video_id>/<id>").delete(delete_comment))
                    .push(Router::with_path("comment/<video_id>").post(create_comment)),
            ),
        );

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(router)
        .await;
}
