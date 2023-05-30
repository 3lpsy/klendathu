mod api;
mod application;
mod cli;
mod config;
mod constants;
mod db;
mod migrations;
mod utils;
use crate::config::Config;
use anyhow::Result;
use clap::ArgMatches;
use db::db::{make_db_url, migrate};
use futures::executor::block_on;
use sea_orm::Database;

async fn run(matches: ArgMatches) -> Result<()> {
    // any top level arg matches should be applied to the config
    let config = Config::from_matches(&matches)?;
    let db_url = make_db_url();
    let db = Database::connect(&db_url).await?;
    let app = application::CliApplication::new(config, db);
    println!("Migrations...");
    migrate(&app).await?;
    match matches.subcommand() {
        Some(("client", sub_matches)) => {
            cli::client::run_command(&app, sub_matches).await?;
        }
        Some(("api", sub_matches)) => {
            cli::api::run_command(&app, sub_matches).await?;
        }
        _ => unimplemented!(),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli::get_matches();
    println!("{:?}", &matches);

    if let Err(err) = block_on(run(matches)) {
        panic!("{}", err);
    }
    Ok(())
}
