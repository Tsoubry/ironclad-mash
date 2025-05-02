use axum::response::Html;
use building_blocks::page;
use index::generate_index;

pub mod building_blocks;
pub mod index;

pub async fn render_index() -> Html<String> {
    let markup = page("Hello!", generate_index());
    Html(markup.into_string())
}
