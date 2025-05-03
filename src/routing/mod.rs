mod health;
mod index;

use axum::{
    Router,
    routing::{get, post},
};
use health::health;
use index::{clicked, get_index, keepalive};

pub fn main_router() -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/clicked", post(clicked))
        .route("/keepalive", get(keepalive))
        .route("/health", get(health))
}
