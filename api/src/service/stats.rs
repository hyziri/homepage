use sea_orm::DatabaseConnection;

use crate::data::stats::StatsRepository;
use crate::error::Error;
use crate::model::stats::StatsDto;

pub async fn get_stats(db: &DatabaseConnection) -> Result<Vec<StatsDto>, Error> {
    let stats_repository = StatsRepository::new(&db);

    let stats = stats_repository.get(vec![], 0, 60).await?;

    let stats_dto = stats.into_iter().map(|s| s.into()).collect();

    Ok(stats_dto)
}
