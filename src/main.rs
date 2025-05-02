use axum::{Router, response::Html, routing::get};
use axum_htmx as _;
use maud::{Markup, html};
use sqlx as _;

const PORT: u16 = 8080;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(render_html));

    let listener = tokio::net::TcpListener::bind(format!("[::]:{PORT}")).await.unwrap();
    println!("Server running at http://localhost:{}", PORT);

    axum::serve(listener, app).await.unwrap();
}

async fn render_html() -> Html<String> {
    let markup = generate_html();
    Html(markup.into_string())
}

fn generate_html() -> Markup {
    html! {
        html {
            head {
                title { "Welcome to Ironclad Mash" }
            }
            body {
                h1 { "Hello, World!" }
                p { "This is a simple HTML page served by Axum and Maud." }
            }
        }
    }
}
