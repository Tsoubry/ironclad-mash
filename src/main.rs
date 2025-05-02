use axum::{Router, routing::get};
use axum_htmx as _;
use sqlx as _;
use templates::render_index;

mod error;
mod templates;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/", get(render_index));

    let listener = tokio::net::TcpListener::bind(format!("::{PORT}")).await?;
    println!("Server running at http://localhost:{}", PORT);

    axum::serve(listener, app).await?;
    Ok(())
}
