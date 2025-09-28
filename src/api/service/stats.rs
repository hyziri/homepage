use sea_orm::DatabaseConnection;

use crate::api::data::stats::StatsRepository;
use crate::api::error::Error;
use crate::model::stats::StatsDto;

pub async fn get_stats(db: &DatabaseConnection) -> Result<Vec<StatsDto>, Error> {
    let stats_repository = StatsRepository::new(&db);

    let filters = vec![];
    let page = 0;
    let page_entries = 60;
    let stats = stats_repository.get(filters, page, page_entries).await?;

    let stats_dto = stats.into_iter().map(|s| s.into()).collect();

    Ok(stats_dto)
}
