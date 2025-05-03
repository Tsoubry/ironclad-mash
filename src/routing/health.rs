use crate::error::IroncladResult;

pub async fn health() -> IroncladResult<String> {
    Ok("up".to_string())
}
