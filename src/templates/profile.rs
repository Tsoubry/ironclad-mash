use maud::{Markup, html};

pub fn generate_profile() -> Markup {
    html! {
        html {
            head {
                title { "Welcome to Ironclad Mash" }
            }
            body {
                h1 { "You're logged in!" }
            }
        }
    }
}
