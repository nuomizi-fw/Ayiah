use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{
    db::entity::{
        prelude::*,
        user::{self},
    },
    error::AyiahError,
};

/// Handles queries (read operations) for user data
pub struct Query;

impl Query {
    /// Find a user by ID
    pub async fn find_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<Option<user::Model>, AyiahError> {
        User::find_by_id(id).one(db).await.map_err(AyiahError::from)
    }

    /// Find a user by username
    pub async fn find_by_username(
        db: &DatabaseConnection,
        username: &str,
    ) -> Result<Option<user::Model>, AyiahError> {
        User::find()
            .filter(user::Column::Username.eq(username))
            .one(db)
            .await
            .map_err(AyiahError::from)
    }

    /// Find a user by email
    pub async fn find_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<Option<user::Model>, AyiahError> {
        User::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
            .map_err(AyiahError::from)
    }

    /// Count all users
    pub async fn count_users(db: &DatabaseConnection) -> Result<u64, AyiahError> {
        User::find().count(db).await.map_err(AyiahError::from)
    }
}
