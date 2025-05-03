use maud::{Markup, html};

pub const CLICK: &str = "click";

pub fn generate_index() -> Markup {
    html! {
        html {
            head {
                title { "Welcome to Ironclad Mash" }
            }
            body {
                h1 { "Hello, World!" }
                p { "This is a simple HTML page served by Axum and Maud." }

                p { "HTMX:" }
                button hx-post="/clicked" hx-trigger=(CLICK) hx-swap="outerHTML" {
                    "Click me!"
                 }

                 div id="keepalive-indicator" hx-get="/keepalive" hx-trigger="every 1s" { }
            }
        }
    }
}
