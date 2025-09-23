mod controller;
mod data;
mod error;
mod model;
mod router;
mod service;
mod task;
mod update;

use axum::Extension;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tokio_cron_scheduler::JobScheduler;
use update::schedule_tasks;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    env_logger::init();

    let contact_email = std::env::var("CONTACT_EMAIL").expect("CONTACT_EMAIL is not set in .env");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let ip = std::env::var("IP").unwrap_or("127.0.0.1".to_string());
    let port = std::env::var("PORT").unwrap_or("8000".to_string());

    let user_agent = format!(
        "{}/{} ({}; +{})",
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
        .expect("Failed to run database migrations");

    let sched = JobScheduler::new()
        .await
        .expect("Failed to create JobScheduler");

    log::info!("Starting Autumn homepage API server");

    schedule_tasks(&sched, &db, &esi_client).await;

    sched.start().await.expect("Failed to start job scheduler");

    let router = router::routes()
        .layer(Extension(esi_client))
        .layer(Extension(db));

    let address = format!("{}:{}", ip, port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
