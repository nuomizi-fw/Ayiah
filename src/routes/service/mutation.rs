use sea_orm::prelude::*;

use crate::{
    db::entity::{
        prelude::*,
        user::{self},
    },
    error::AyiahError,
};

/// Handles mutations (write operations) for user data
pub struct Mutation;

impl Mutation {
    /// Create a new user
    pub async fn create_user(
        db: &DatabaseConnection,
        new_user: user::ActiveModel,
    ) -> Result<user::Model, AyiahError> {
        User::insert(new_user)
            .exec_with_returning(db)
            .await
            .map_err(AyiahError::from)
    }

    /// Update a user
    pub async fn update_user(
        db: &DatabaseConnection,
        user: user::ActiveModel,
    ) -> Result<user::Model, AyiahError> {
        User::update(user).exec(db).await.map_err(AyiahError::from)
    }
}
