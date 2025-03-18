use sea_orm_migration::prelude::*;

use super::m20250317_074800_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserPreferences::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserPreferences::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserPreferences::UserId).uuid().not_null())
                    .col(ColumnDef::new(UserPreferences::Key).string().not_null())
                    .col(ColumnDef::new(UserPreferences::Value).json().not_null())
                    .col(ColumnDef::new(UserPreferences::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(UserPreferences::UpdatedAt).timestamp().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_preferences_user")
                            .from(UserPreferences::Table, UserPreferences::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        // Unique constraint on user_id + key
        manager
            .create_index(
                Index::create()
                    .name("idx_user_pref_key")
                    .table(UserPreferences::Table)
                    .col(UserPreferences::UserId)
                    .col(UserPreferences::Key)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserPreferences::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum UserPreferences {
    Table,
    Id,
    UserId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
}

