use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
pub struct ErrorDto {
    pub error: String,
}
