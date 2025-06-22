use axum::response::Html;
use building_blocks::page;
use index::generate_index;
use login::generate_login;
use profile::generate_profile;

pub mod building_blocks;
pub mod index;
pub mod login;
pub mod profile;

pub async fn render_index() -> Html<String> {
    let markup = page("Hello!", generate_index());
    Html(markup.into_string())
}

pub async fn render_login() -> Html<String> {
    let markup = page("Login!", generate_login());
    Html(markup.into_string())
}

pub async fn render_profile() -> Html<String> {
    let markup = page("Profile", generate_profile());
    Html(markup.into_string())
}
