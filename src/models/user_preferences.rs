use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

/// Request model for setting a user preference
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SetUserPreferencePayload {
    /// Preference key (name)
    pub key: String,
    /// Preference value (can be any valid JSON value)
    pub value: Value,
}
