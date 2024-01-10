use axum::response::{IntoResponse, Response};
use http::StatusCode;

#[derive(Debug)]
pub struct ApiError(anyhow::Error);

// impl From<sqlx::Error> for ApiError {
//     fn from(value: sqlx::Error) -> Self {
//         Self::DatabaseError(value)
//     }
// }

// Tell axum how to convert `ApiError` into a response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("Application error: {:#}", self.0);

        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, ApiError>`. That way you don't need to do that manually.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
