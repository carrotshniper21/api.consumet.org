use consumet_api_rs::providers::movies;
use serde::{Deserialize, Serialize};

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

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

async fn flixhq() -> (StatusCode, Test) {
    let flixhq = movies::FlixHQ;

    let deez = flixhq.search("hi", None).await.unwrap();
    let result = Test {
        id: deez.results[0].id.clone().unwrap(),
    };

    (StatusCode::OK, result)
}
