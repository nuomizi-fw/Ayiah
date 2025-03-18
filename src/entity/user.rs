use sea_orm::entity::prelude::*;
use utoipa::ToSchema;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, ToSchema)]
#[sea_orm(table_name = "user")]
#[schema(as = User)]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    #[sea_orm(unique)]
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
    pub avatar: Option<String>,
    pub is_admin: bool,
    #[schema(value_type = DateTime)]
    pub created_at: DateTimeUtc,
    #[schema(value_type = DateTime)]
    pub updated_at: DateTimeUtc,
    #[schema(value_type = DateTime)]
    pub last_login_at: Option<DateTimeUtc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::user_preferences::Entity")]
    UserPreferences,
}

impl Related<super::user_preferences::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserPreferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
