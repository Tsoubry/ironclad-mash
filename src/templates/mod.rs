use axum::response::Html;
use index::generate_index;

pub mod index;

pub async fn render_index() -> Html<String> {
    let markup = generate_index();
    Html(markup.into_string())
}
