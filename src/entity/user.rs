use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "user")]
#[schema(as = User)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub hashed_password: String,
    pub salt: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub is_admin: bool,
    #[schema(value_type = DateTime)]
    pub created_at: DateTimeWithTimeZone,
    #[schema(value_type = DateTime)]
    pub updated_at: DateTimeWithTimeZone,
    #[schema(value_type = DateTime)]
    pub last_login_at: Option<DateTimeWithTimeZone>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
