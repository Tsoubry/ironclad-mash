use axum::{Router, routing::get};
use axum_htmx as _;
use sqlx as _;
use templates::render_index;

mod templates;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(render_index));

    let listener = tokio::net::TcpListener::bind(format!("[::]:{PORT}")).await.unwrap();
    println!("Server running at http://localhost:{}", PORT);

    axum::serve(listener, app).await.unwrap();
}
