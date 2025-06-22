use maud::{Markup, html};

pub fn generate_login() -> Markup {
    html! {
        html {
            head {
                title { "Login to Ironclad Mash" }
            }
            body {
                h1 { "Login" }
            }
        }
    }
}
