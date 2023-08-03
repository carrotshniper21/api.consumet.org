use crate::models::{FlixhqSearch, ProviderInfo};
use axum::{extract::Query, http::StatusCode, routing::get, Json, Router};
use consumet_api_rs::{
    models::{IMovieResult, ISearch},
    providers::movies,
};

pub async fn mount() -> Router {
    Router::new()
        .route("/", get(flixhq_home))
        .route("/search", get(flixhq))
}

pub async fn flixhq_home() -> (StatusCode, Json<ProviderInfo>) {
    (
        StatusCode::OK,
        Json(ProviderInfo {
            intro: String::from(
                r#"Welcome to the flixhq provider: check out the provider's website @ https://flixhq.to/"#,
            ),
            routes: vec![
                String::from("/search"),
                String::from("/info"),
                String::from("/watch"),
            ],
            documentation: String::from("https://docs.consumet.org/#tag/flixhq"),
        }),
    )
}

pub async fn flixhq(
    route_params: Query<FlixhqSearch>,
) -> (StatusCode, Json<ISearch<IMovieResult>>) {
    let flixhq = movies::FlixHQ;

    let deez = flixhq
        .search(&route_params.query, route_params.page)
        .await
        .unwrap();

    (StatusCode::OK, Json(deez))
}
