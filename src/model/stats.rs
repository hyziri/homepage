use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
/// A daily entry for the member count of an EVE Online corporation
pub struct StatsDto {
    /// ID for the stats entry
    pub id: i32,
    /// ID for the EVE Online corporation
    pub corporation_id: i64,
    /// The member count for the entry
    pub member_count: i64,
    /// The timestamp for the entry
    pub date: DateTime<Utc>,
}

impl Default for StatsDto {
    fn default() -> Self {
        StatsDto {
            id: 0,
            corporation_id: 0,
            member_count: 0,
            date: Utc::now(),
        }
    }
}
