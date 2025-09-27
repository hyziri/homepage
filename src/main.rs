#![allow(non_snake_case)]

mod model;
mod web;

#[cfg(feature = "server")]
mod api;

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(web::App);
}

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use api::update::schedule_tasks;
    use axum::routing::*;
    use dioxus::prelude::*;
    use dioxus_logger::tracing::{info, Level};
    use migration::{Migrator, MigratorTrait};
    use sea_orm::{ConnectOptions, Database};
    use tokio_cron_scheduler::JobScheduler;

    use crate::api::router::AppState;

    dotenvy::dotenv().ok();

    let contact_email = std::env::var("CONTACT_EMAIL").expect("CONTACT_EMAIL is not set in .env");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");

    let user_agent = format!(
        "{}/{} ({}; +{}) ",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        contact_email,
        env!("CARGO_PKG_REPOSITORY")
    );
    let esi_client = eve_esi::Client::new(&user_agent).expect("Failed to build ESI client");

    let mut opt = ConnectOptions::new(database_url);
    opt.sqlx_logging(false);

    let db = Database::connect(opt)
        .await
        .expect("Failed to connect to the database");

    Migrator::up(&db, None)
        .await
        .expect("Failed to run migrations");

    let sched = JobScheduler::new()
        .await
        .expect("Failed to create JobScheduler");

    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("Starting server");

    schedule_tasks(&sched, &db, &esi_client).await;

    sched.start().await.expect("Failed to start scheduler");

    let state = AppState { db: db };

    let router = Router::new()
        .serve_dioxus_application(ServeConfigBuilder::default(), web::App)
        .nest("/api", api::router::routes())
        .with_state(state);

    let router = router.into_make_service();
    let address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
