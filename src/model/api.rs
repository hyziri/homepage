use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(utoipa::ToSchema))]
/// The response when an error occurs with an API request
pub struct ErrorDto {
    /// The error message
    pub error: String,
}
