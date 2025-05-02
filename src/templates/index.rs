use maud::{Markup, html};

pub fn generate_index() -> Markup {
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
