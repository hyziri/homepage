use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use crate::api::router::AppState;

#[utoipa::path(
    get,
    path = "/api/stats",
    tag = "Autumn Corporation Stats",
    responses(
        (status = 200, description = "Entries for The Order of Autumn's member count for the last 60 days", body = StatsDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    )
)]
pub async fn get_stats(State(state): State<AppState>) -> Response {
    use crate::api::service::stats::get_stats;

    match get_stats(&state.db).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(err) => err.into_response(),
    }
}
