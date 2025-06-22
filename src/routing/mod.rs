mod health;
mod index;
mod login;
mod profile;

use axum::{
    Router,
    routing::{get, post},
};
use health::health;
use index::{clicked, get_index, keepalive};
use login::get_login;
use profile::get_profile;

pub fn main_router() -> Router {
    Router::new()
        .route("/", get(get_index))
        .route("/clicked", post(clicked))
        .route("/keepalive", get(keepalive))
        .route("/health", get(health))
        .route("/login", get(get_login))
        .route("/profile", get(get_profile))
}
