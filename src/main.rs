use axum::{http::StatusCode, routing::get, Json, Router};
use std::str::FromStr;
mod models;
mod routes;
use models::ResponseError;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mut port = std::env::var("PORT").unwrap();
    port = if port.is_empty() {
        "8080".to_string()
    } else {
        port
    };

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .nest("/movies", routes::movies::mount().await)
        .fallback(fallback_func);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = std::net::SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn home() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Welcome to consumet api rust! ")
}

async fn fallback_func() -> (StatusCode, Json<ResponseError>) {
    (
        StatusCode::NOT_FOUND,
        Json(ResponseError {
            message: String::new(),
            error: String::from("page not found"),
        }),
    )
}
