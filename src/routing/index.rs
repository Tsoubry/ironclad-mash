use axum::response::Html;
use axum_htmx::HxRequest;
use maud::html;

use crate::{
    error::{IroncladError, IroncladResult},
    templates::render_index,
};

pub async fn get_index() -> IroncladResult<Html<String>> {
    Ok(render_index().await)
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
