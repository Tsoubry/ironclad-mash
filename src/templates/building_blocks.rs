use maud::{DOCTYPE, Markup, html};

fn header(page_title: &str) -> Markup {
    html! {
        (DOCTYPE)
        meta charset="utf-8";
        title { (page_title) }
        script src="https://unpkg.com/htmx.org@2.0.4" { };
    }
}

fn footer() -> Markup {
    html! {
        footer {
            a href="rss.atom" { "RSS Feed" }
        }
    }
}

pub fn page(title: &str, greeting_box: Markup) -> Markup {
    html! {
        (header(title))
        (greeting_box)
        (footer())
    }
}
