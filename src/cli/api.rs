use anyhow::Result;
use clap::{ArgMatches, Command};

use crate::api::router::make_router;

// use crate::api::ApiClient;
use crate::application::CliApplication;

pub fn make_command() -> Command {
    Command::new("api")
        .about("Manage api")
        .subcommand(Command::new("run").about("Run api"))
        .subcommand(Command::new("status").about("Get api status"))
}

pub async fn run_command(app: &CliApplication, matches: &ArgMatches) -> Result<()> {
    let router = make_router().await;
    axum::Server::bind();

    unimplemented!();
    // match matches.subcommand() {
    //     Some(("list", sub_matches)) => {
    //         unimplemented!()
    //         // let api = ApiClient::new(app.api_url())?;
    //         // let clients = api.clients_list().await?;
    //         // for client in clients {
    //         //     println!("Client: {:?}", client);
    //         // }
    //     }
    //     _ => unimplemented!(),
    // };
}
