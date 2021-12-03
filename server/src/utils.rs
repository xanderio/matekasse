use axum::{http::StatusCode, response::IntoResponse, Json};
use eyre::Report;
use sea_orm::{DbErr, TransactionError};
use serde_json::json;

pub(crate) type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AppError {
    #[error("database error")]
    DbErr(#[from] DbErr),
    #[error(transparent)]
    Transaction(eyre::Report),
    #[error("already exists")]
    Conflict,
    #[error("not found")]
    NotFount,
    #[error(transparent)]
    ParseError(#[from] std::num::ParseIntError),
    #[error("{0:?}")]
    Error(#[from] eyre::Error),
}

impl<T> From<TransactionError<T>> for AppError
where
    T: std::error::Error + Send + Sync + 'static,
{
    fn from(err: TransactionError<T>) -> Self {
        match err {
            TransactionError::Connection(err) => AppError::DbErr(err),
            TransactionError::Transaction(err) => AppError::Transaction(Report::new(err)),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::http::Response<axum::body::BoxBody> {
        // special case unique constraint failure and return 409
        if let AppError::DbErr(DbErr::Exec(ref msg)) = self {
            // this is a ugly hack, but i'm not sure how to clean this up :(
            if msg.contains("UNIQUE constraint failed") {
                return AppError::Conflict.into_response();
            }
        };

        let message = format!("{:?}", self);
        let status = match self {
            AppError::DbErr(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Transaction(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::NotFount => StatusCode::NOT_FOUND,
            AppError::ParseError(_) => StatusCode::BAD_REQUEST,
            AppError::Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(json!({ "status": "error", "message": message }));

        (status, body).into_response()
    }
}
