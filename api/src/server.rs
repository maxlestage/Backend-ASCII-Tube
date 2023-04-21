use crate::handlers::{
    create_comment, delete_comment, delete_video, get_comment, get_video, hello_world, sign_up,
    upload_video,
};
use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use db::db_connection::db_connection;
use migration::{Migrator, MigratorTrait};
use salvo::jwt_auth::HeaderFinder;
use salvo::{__private::tracing, cors::Cors, prelude::*};
use sea_orm::DatabaseConnection;

pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");
    let db_connect: DatabaseConnection = db_connection().await.expect("Error");
    Migrator::up(&db_connect, None).await.expect("Error");
    let _auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let cors_handler = Cors::builder()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "DELETE"])
        .build();

    let router = Router::new()
        .get(hello_world)
        .push(Router::with_path("video/upload").post(upload_video))
        .push(Router::with_path("signup").hoop(cors_handler).post(sign_up))
        .push(Router::with_path("login").post(sign_in))
        .push(Router::with_path("video/<id>").get(get_video))
        .push(Router::with_path("video/<id>").delete(delete_video))
        .push(Router::with_path("video/<video_id>/<user_id>").post(create_comment))
        .push(Router::with_path("video/<video_id>/comment/").get(get_comment))
        .push(Router::with_path("video/<video_id>/comment/<id>").delete(delete_comment))

        // .push(
        //     Router::with_path("upload")
        //         .hoop(auth_handler)
        //         .post(upload_video)
        //         .push(
        //             Router::new()
        //                 .path("hello")
        //                 // .hoop(auth_handler)
        //                 .get(hello_world)
        //                 .push(Router::with_path("<id>").get(hello_by_id)),
        //         ),
        // );
        ;

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(router)
        .await;
}
