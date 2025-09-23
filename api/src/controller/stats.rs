use axum::Extension;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use sea_orm::DatabaseConnection;

use crate::model::error::ErrorDto;
use crate::model::stats::StatsDto;

pub const STATS_TAG: &str = "AutumnCorporationStats";

#[utoipa::path(
    get,
    path = "/",
    tag = STATS_TAG,
    responses(
        (status = 200, description = "Retrieved Autumn corporation stats successfully", body = StatsDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
)]
pub async fn get_stats(Extension(db): Extension<DatabaseConnection>) -> Response {
    use crate::service::stats::get_stats;

    match get_stats(&db).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(err) => err.into_response(),
    }
}
