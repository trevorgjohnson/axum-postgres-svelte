use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::Error as SqlxError;

pub enum CustomError {
    InternalServerError(String),
    NotFound,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            CustomError::InternalServerError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(format!("Something went wrong...\n\nError: {}", e)),
            ),
            CustomError::NotFound => (
                StatusCode::NOT_FOUND,
                Json("Unable to find that user".to_string()),
            ),
        };

        body.into_response()
    }
}

impl From<SqlxError> for CustomError {
    fn from(error: SqlxError) -> Self {
        match error {
            SqlxError::RowNotFound => Self::NotFound,
            err => Self::InternalServerError(err.to_string()),
        }
    }
}

pub async fn internal_error(err: impl std::error::Error) -> impl IntoResponse {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(format!("Something went wrong...\n\nError: {}", err)),
    )
}
