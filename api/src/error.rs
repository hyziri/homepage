use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::model::error::ErrorDto;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    DbErr(#[from] sea_orm::DbErr),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::DbErr(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorDto {
                    error: "Internal Server Error".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
