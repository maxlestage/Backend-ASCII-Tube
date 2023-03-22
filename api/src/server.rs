use auth::jwt_auth::{sign_in, JwtClaims, SECRET_KEY};
use salvo::jwt_auth::HeaderFinder;
use salvo::{__private::tracing, prelude::*};

use crate::handlers::{hello_by_id, hello_world, sign_up};

// #[tokio::main]
pub async fn main() {
    tracing_subscriber::fmt().init();
    tracing::info!("Listening on http://0.0.0.0:7878");

    let auth_handler: JwtAuth<JwtClaims> = JwtAuth::new(SECRET_KEY.to_owned())
        .with_finders(vec![Box::new(HeaderFinder::new())])
        .with_response_error(true);

    let router = Router::new()
        .get(hello_world)
        .push(Router::with_path("signup").post(sign_up))
        .push(Router::with_path("signin").post(sign_in))
        .push(
            Router::new()
                .path("hello")
                .hoop(auth_handler)
                .get(hello_world)
                .push(Router::with_path("<id>").get(hello_by_id)),
        );

    // Server Ready
    Server::new(TcpListener::bind("0.0.0.0:7878"))
        .serve(router)
        .await;
}
