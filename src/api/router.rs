use axum::routing::get;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::api::controller;
use crate::model;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

pub fn routes() -> Router<AppState> {
    #[derive(OpenApi)]
    #[openapi(
        paths(controller::stats::get_stats),
        components(schemas(model::api::ErrorDto, model::stats::StatsDto)),
        tags((name = "Autumn Homepage API", description = "API provider for The Order of Autumn's homepage"))
    )]
    struct ApiDoc;

    let stats_routes = Router::new().route("/api/stats", get(controller::stats::get_stats));

    let routes = Router::new()
        .merge(stats_routes)
        .merge(SwaggerUi::new("/api/docs").url("/api/docs/openapi.json", ApiDoc::openapi()));

    routes
}
