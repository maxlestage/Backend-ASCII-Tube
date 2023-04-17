use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};

use crate::handlers::{hello_by_id, hello_world, sign_up, upload_video};
use salvo::jwt_auth::HeaderFinder;
use salvo::{__private::tracing, prelude::*};

pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let router = Router::new()
        .get(hello_world)
        .push(Router::with_path("upload").post(upload_video))
        .push(Router::with_path("signup").post(sign_up))
        .push(Router::with_path("login").post(sign_in))
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
