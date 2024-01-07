#[derive(Debug)]
pub enum ApiError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for ApiError {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;
