use anyhow::Context;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    message: &'static str,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .context("failed to bind TCP listener")?;
    axum::serve(listener, app)
        .await
        .context("axum::server failed")?;

    Ok(())
}

async fn hello_json() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Hello, world!",
    };

    (StatusCode::OK, Json(response))
}
