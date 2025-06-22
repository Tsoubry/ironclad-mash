use axum::response::Html;

use crate::{error::IroncladResult, templates::render_login};

pub async fn get_login() -> IroncladResult<Html<String>> {
    Ok(render_login().await)
}
