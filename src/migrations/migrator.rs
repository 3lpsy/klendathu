pub use sea_orm_migration::prelude::*;

use crate::migrations::{
    m20230525_000001_create_clients, m20230525_000002_create_projects,
    m20230525_000003_create_scopes, m20230525_000004_create_assets,
};

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230525_000001_create_clients::Migration),
            Box::new(m20230525_000002_create_projects::Migration),
            Box::new(m20230525_000003_create_scopes::Migration),
            Box::new(m20230525_000004_create_assets::Migration),
        ]
    }
}
