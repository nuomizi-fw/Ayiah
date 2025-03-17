use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Schema alias for user::Model
#[derive(Serialize, Deserialize, ToSchema)]
#[schema(as = crate::entity::user::Model)]
pub struct Model {
    id: String,
}

/// Request model for creating a new user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserRequest {
    /// Unique username (required)
    pub username: String,
    /// User's email address (required)
    pub email: String,
    /// User's password (required, will be hashed)
    pub password: String,
    /// Display name (optional)
    pub display_name: Option<String>,
    /// Avatar URL (optional)
    pub avatar: Option<String>,
}

/// Request model for updating an existing user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateUserRequest {
    /// User's email address (optional)
    pub email: Option<String>,
    /// User's password (optional, will be hashed)
    pub password: Option<String>,
    /// Display name (optional)
    pub display_name: Option<String>,
    /// Avatar URL (optional)
    pub avatar: Option<String>,
}
