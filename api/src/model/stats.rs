use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct StatsDto {
    pub corporation_id: i64,
    pub member_count: i64,
    pub date: DateTime<Utc>,
}

use entity::stats::Model;

impl From<Model> for StatsDto {
    fn from(model: Model) -> Self {
        StatsDto {
            corporation_id: model.corporation_id,
            member_count: model.member_count,
            date: DateTime::<Utc>::from_naive_utc_and_offset(model.date, Utc),
        }
    }
}
