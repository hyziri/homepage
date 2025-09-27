use axum::routing::get;
use axum::Router;

use crate::api::controller;

#[derive(Clone)]
pub struct AppState {
    pub db: sea_orm::DatabaseConnection,
}

pub fn routes() -> Router<AppState> {
    let stats_routes = Router::new().route("/stats", get(controller::stats::get_stats));

    let routes = Router::new().merge(stats_routes);

    routes
}
