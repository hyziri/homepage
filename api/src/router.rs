use axum::Router;
use utoipa::OpenApi;
use utoipa_axum::{router::OpenApiRouter, routes};
use utoipa_swagger_ui::SwaggerUi;

use crate::controller;

pub fn routes() -> Router {
    #[derive(OpenApi)]
    #[openapi(info(
        title = "Autumn API",
        description = "Autumn homepage & tools API"
    ), tags(
        (name = controller::stats::STATS_TAG, description = "Historical member stats for Autumn corporations")
    ))]
    struct ApiDoc;

    let stats_router = OpenApiRouter::new().routes(routes!(controller::stats::get_stats));

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/stats", stats_router)
        .split_for_parts();

    let router = router.merge(SwaggerUi::new("/").url("/openapi.json", api));

    router
}
