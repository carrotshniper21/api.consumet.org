mod models;
mod routes;
mod utils;

use std::{
    fmt::Display,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    panic,
    time::Duration,
};

use anyhow::{Context, Result};
use axum::{
    extract::{ConnectInfo, Request},
    http::StatusCode,
    middleware::{from_fn_with_state, Next},
    response::Response,
    routing::get,
    Json, Router,
};
use serde_json::json;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    let result = init_tracing();
    if let Err(ref error) = result {
        log_error(error);
        return;
    };

    // Replace the default panic hook with one that uses structured logging at ERROR level.
    panic::set_hook(Box::new(|panic| error!(%panic, "process panicked")));

    // Run and log any error.
    if let Err(ref error) = run().await {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "process exited with ERROR"
        );
    }
}

fn init_tracing() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().json())
        .try_init()
        .context("initialize tracing subscriber")?;

    Ok(())
}

fn log_error(error: &impl Display) {
    let now = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
    let error = serde_json::to_string(&json!({
        "timestamp": now,
        "level": "ERROR",
        "message": "process exited with ERROR",
        "error": format!("{error:#}")
    }));
    // Not using `eprintln!`, because `tracing_subscriber::fmt` uses stdout by default.
    println!("{}", error.unwrap());
}

async fn home() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Welcome to consumet api rust! ")
}

async fn fallback_func() -> (StatusCode, Json<models::ResponseError>) {
    (
        StatusCode::NOT_FOUND,
        Json(models::ResponseError {
            message: String::new(),
            error: String::from("page not found"),
        }),
    )
}

async fn shutdown_signal(shutdown_timeout: Option<Duration>) {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;
    if let Some(shutdown_timeout) = shutdown_timeout {
        sleep(shutdown_timeout).await;
    }
}

async fn run() -> Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse::<u16>()?;

    info!("Server started on http://localhost:{:#?}", port);

    // build our application with a route
    let app = Router::new()
        .route("/", get(home))
        .nest("/movies", routes::movies::mount().await)
        .route_layer(from_fn_with_state("state", requests_stats))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
        )
        .fallback(fallback_func);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = TcpListener::bind(&addr).await.context("bind TcpListener")?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(None))
    .await
    .context("run server")
}

async fn requests_stats(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    info!("{}:{}", addr.ip(), addr.port());
    next.run(req).await
}
