use consumet_api_rs::providers::movies;
use consumet_api_rs::models::{IMovieResult, ISearch};
use serde::{Deserialize, Serialize};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

#[derive(Debug, Deserialize, Serialize)]
struct Test {
    id: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(flixhq));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn flixhq() -> (StatusCode, Json<ISearch<IMovieResult>>) {
    let flixhq = movies::FlixHQ;

    let deez = flixhq.search("hi", None).await.unwrap();

    (StatusCode::OK, Json(deez))
}
