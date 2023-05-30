use sea_orm::DatabaseConnection;

use crate::config::Config;

pub trait Application {
    fn config(&self) -> &Config;
    fn db(&self) -> &DatabaseConnection;
}

pub struct CliApplication {
    pub db: DatabaseConnection,
    pub config: Config,
}

impl CliApplication {
    pub fn new(config: Config, db: DatabaseConnection) -> Self {
        Self { config, db }
    }
}

impl Application for CliApplication {
    fn config(&self) -> &Config {
        &self.config
    }
    fn db(&self) -> &DatabaseConnection {
        &self.db
    }
}
