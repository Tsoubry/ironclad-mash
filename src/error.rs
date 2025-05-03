use axum::{http::StatusCode, response::IntoResponse};

#[derive(Debug)]
pub enum IroncladError {
    ServerError,
    BadRequest,
}

impl std::fmt::Display for IroncladError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IroncladError::ServerError => write!(f, "Something's wrong"),
            IroncladError::BadRequest => write!(f, "Bad request"),
        }
    }
}

impl std::error::Error for IroncladError {}

pub type IroncladResult<T> = Result<T, IroncladError>;

impl IntoResponse for IroncladError {
    fn into_response(self) -> axum::response::Response {
        let error = match self {
            IroncladError::ServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
            IroncladError::BadRequest => (StatusCode::BAD_REQUEST, "Bad request"),
        };

        error.into_response()
    }
}
