use sea_orm::DatabaseConnection;
use tokio_cron_scheduler::{Job, JobScheduler};

use super::task::stats::task_update_corporation_stats;

pub async fn schedule_tasks(
    sched: &JobScheduler,
    db: &DatabaseConnection,
    esi_client: &eve_esi::Client,
) {
    task_update_corporation_stats(&esi_client, db).await;

    let db_clone = db.clone();
    let esi_client = esi_client.clone();

    sched
        .add(
            Job::new_async("0 37 * * * *", move |_uuid, _l| {
                let esi_client = esi_client.clone();
                let db = db_clone.clone();
                Box::pin(async move {
                    task_update_corporation_stats(&esi_client, &db).await;
                })
            })
            .expect("Failed to create job"),
        )
        .await
        .expect("Failed to add job to scheduler");
}
