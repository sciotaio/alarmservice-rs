use sea_orm_migration::{async_trait, MigrationTrait, MigratorTrait};

pub mod v1_001_create_tables;
pub mod v1_002_add_data;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(v1_001_create_tables::Migration),
            Box::new(v1_002_add_data::Migration),
        ]
    }
}
