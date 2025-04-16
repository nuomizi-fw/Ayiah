use sea_orm_migration::{MigrationTrait, MigratorTrait};

mod m20250317_074800_create_user;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250317_074800_create_user::Migration),
        ]
    }
}
