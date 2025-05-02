use axum::{
    Router,
    routing::{get, post},
};
use axum_htmx as _;
use axum_htmx::AutoVaryLayer;
use routes::index;
use sqlx as _;

mod error;
mod routes;
mod templates;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .route("/clicked", post(routes::clicked))
        .layer(AutoVaryLayer);

    let listener = tokio::net::TcpListener::bind(format!("[::]:{PORT}")).await?;
    println!("Server running at http://localhost:{}", PORT);

    axum::serve(listener, app).await?;
    Ok(())
}
