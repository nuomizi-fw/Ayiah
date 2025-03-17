use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// Schema alias for user_preferences::Model
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = entity::user_preferences::Model)]
pub struct Model {
    id: String,
}

/// Request model for setting a user preference
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetUserPreferenceRequest {
    /// Preference key (name)
    pub key: String,
    /// Preference value (can be any valid JSON value)
    pub value: Value,
}
