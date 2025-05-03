use axum::response::Html;
use axum_htmx::{HxEvent, HxRequest, HxResponseTrigger};
use maud::html;
use serde_json::json;

use crate::{
    error::{IroncladError, IroncladResult},
    templates::render_index,
};

pub const CLICK: &str = "click";

pub async fn index() -> (HxResponseTrigger, Html<String>) {
    let event = HxEvent::new_with_data(
        "my-event",
        // May be any object that implements `serde::Serialize`
        json!({"level": "info", "message": {
            "title": "Hello, world!",
            "body": "This is a test message.",
        }}),
    )
    .unwrap();

    (HxResponseTrigger::normal([event]), render_index().await)
}

pub async fn clicked(HxRequest(clicked): HxRequest) -> IroncladResult<Html<String>> {
    if clicked {
        let markup = html!(
            {
                p { "clicked" }
            }
        );
        return Ok(Html(markup.into_string()));
    }

    Err(IroncladError::BadRequest)
}
