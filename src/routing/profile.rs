use axum::response::Html;

use crate::{error::IroncladResult, templates::render_profile};

pub async fn get_profile() -> IroncladResult<Html<String>> {
    Ok(render_profile().await)
}
