use axum::response::{Html, IntoResponse};
use axum_htmx::{HxResponseTrigger, HxTrigger};
use maud::html;

use crate::templates::render_index;

pub const CLICK: &str = "click";

pub async fn index() -> (HxResponseTrigger, Html<String>) {
    (HxResponseTrigger::normal([CLICK]), render_index().await)
}

pub async fn clicked(HxTrigger(trigger): HxTrigger) -> impl IntoResponse {
    let result = match trigger {
        Some(event) => event,
        _ => "Error!".to_string(),
    };

    let markup = html!(
        {
            p { "Event was: " (result) }
        }
    );

    Html(markup.into_string())
}
