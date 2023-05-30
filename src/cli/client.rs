use anyhow::Result;
use clap::{ArgMatches, Command};

// use crate::api::ApiClient;
use crate::application::CliApplication;

pub fn make_command() -> Command {
    Command::new("client")
        .about("Manage clients")
        .subcommand(Command::new("list").about("List clients"))
        .subcommand(Command::new("create").about("Create new client"))
}

pub async fn run_command(app: &CliApplication, matches: &ArgMatches) -> Result<()> {
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
