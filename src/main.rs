use axum::Router;
use axum_htmx::AutoVaryLayer;
use routing::main_router;
use serde_json as _;
use sqlx as _;

mod error;
mod routing;
mod templates;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().merge(main_router()).layer(AutoVaryLayer);

    let listener = tokio::net::TcpListener::bind(format!("[::]:{PORT}")).await?;
    println!("Server running at http://localhost:{}", PORT);

    axum::serve(listener, app).await?;
    Ok(())
}
