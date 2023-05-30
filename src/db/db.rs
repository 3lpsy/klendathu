use crate::application::Application;
use crate::config::Environment;
use crate::migrations::Migrator;
use crate::utils::touch;
use anyhow::Result;
use sea_orm_migration::prelude::*;
use std::path::PathBuf;

pub async fn migrate(app: &dyn Application) -> Result<()> {
    let db = app.db();
    let schema_manager = SchemaManager::new(db);
    if let Some(Environment::Local) = app.config().environment {
        Migrator::fresh(db).await?;
        // println!(
        //     "Client Table Exists: {:?}",
        //     schema_manager.has_table("clients").await?
        // );
        // println!(
        //     "Projects Table Exists: {:?}",
        //     schema_manager.has_table("projects").await?
        // );
        // println!(
        //     "Scopes Table Exists: {:?}",
        //     schema_manager.has_table("scopes").await?
        // );
        // println!(
        //     "Assets Table Exists: {:?}",
        //     schema_manager.has_table("assets").await?
        // );
    }
    Ok(())
}

pub fn make_db_url() -> String {
    let mut db_path = std::env::current_dir().unwrap();
    db_path.push(PathBuf::from("test.db"));
    touch(&db_path);
    String::from("sqlite://") + db_path.to_str().unwrap()
}
