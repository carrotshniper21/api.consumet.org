use crate::routes::movies;
use axum::{http::StatusCode, routing::get, Router};

pub async fn mount() -> Router {
    Router::new()
        .route("/", get(home))
        .nest("/flixhq", movies::flixhq::mount().await)
        .nest("/dramacool", movies::dramacool::mount().await)
}

pub async fn home() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Welcome to Consumet Movies and TV Shows")
}
