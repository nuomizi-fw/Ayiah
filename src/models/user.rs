use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthPayload {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct AuthBody {
    pub token: String,
    pub token_type: String,
}

impl AuthBody {
    pub fn new(token: String) -> Self {
        Self {
            token,
            token_type: "Bearer".to_string(),
        }
    }
}

/// Request model for creating a new user
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateUserPayload {
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
pub struct UpdateUserPayload {
    /// User's email address (optional)
    pub email: Option<String>,
    /// User's password (optional, will be hashed)
    pub password: Option<String>,
    /// Display name (optional)
    pub display_name: Option<String>,
    /// Avatar URL (optional)
    pub avatar: Option<String>,
}
