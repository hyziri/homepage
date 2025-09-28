use chrono::{Duration, Utc};
use sea_orm::{ColumnTrait, DatabaseConnection};

use crate::api::{constant::AUTUMN_CORPORATION_IDS, data::stats::StatsRepository};

pub async fn task_update_corporation_stats(db: &DatabaseConnection, esi_client: &eve_esi::Client) {
    let corporation_ids = AUTUMN_CORPORATION_IDS.to_vec();

    update_corporation_stats(db, &esi_client, corporation_ids).await;
}

pub async fn update_corporation_stats(
    db: &DatabaseConnection,
    esi_client: &eve_esi::Client,
    corporation_ids: Vec<i64>,
) {
    log::info!("Running Autumn corporation stats update task");

    let stats_repository = StatsRepository::new(db);

    let mut updated_entry_count: i32 = 0;

    for corporation_id in corporation_ids {
        let filters = vec![entity::stats::Column::CorporationId.eq(corporation_id)];

        let recent_entries = match stats_repository.get(filters, 0, 1).await {
            Ok(entries) => entries,
            Err(error) => {
                log::error!(
                    "Failed to retrieve entries for corporation {} from stats table in database: {}",
                    corporation_id,
                    error
                );

                continue;
            }
        };

        if !recent_entries.is_empty() {
            let recent_entry = &recent_entries[0];

            let now = Utc::now();
            if now - recent_entry.date < Duration::hours(24) {
                continue;
            }
        }

        let corporation = match esi_client
            .corporation()
            .get_corporation_information(corporation_id)
            .await
        {
            Ok(corporation) => corporation,
            Err(error) => {
                log::error!(
                    "Failed to retrieve info for corporation {} from ESI: {}",
                    corporation_id,
                    error
                );

                continue;
            }
        };

        match stats_repository
            .create(corporation_id, corporation.member_count)
            .await
        {
            Ok(_) => updated_entry_count += 1,
            Err(error) => {
                log::error!(
                    "Failed to save corporation {} to database: {}",
                    corporation_id,
                    error
                );

                continue;
            }
        };
    }

    if updated_entry_count > 0 {
        log::info!(
            "Updated {} Autumn corporation stats entries",
            updated_entry_count
        );
    } else {
        log::info!("Ran Autumn corporation stats update task, no updates needed at this moment.");
    }
}
