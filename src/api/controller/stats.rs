use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

use crate::api::router::AppState;

pub async fn get_stats(State(state): State<AppState>) -> Response {
    use crate::api::service::stats::get_stats;

    match get_stats(&state.db).await {
        Ok(stats) => (StatusCode::OK, Json(stats)).into_response(),
        Err(err) => err.into_response(),
    }
}
