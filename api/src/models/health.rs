use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    /// Service status
    #[schema(example = "ok")]
    pub status: String,

    /// Whether the solver model is loaded
    #[schema(example = true)]
    pub model_loaded: bool,

    /// API version string
    #[schema(example = "0.1.0")]
    pub version: String,
}

impl Default for HealthResponse {
    fn default() -> Self {
        Self {
            status: "ok".to_string(),
            model_loaded: true,
            version: "0.1.0".to_string(),
        }
    }
}
