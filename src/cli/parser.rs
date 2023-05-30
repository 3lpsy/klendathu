use crate::cli::{api, client};
use crate::config::Environment;
use clap::{value_parser, Arg, ArgAction, ArgMatches, Command};

pub fn get_matches() -> ArgMatches {
    Command::new("klendathu")
        .about("Remember Klendathu!")
        .version("0.1.0")
        .arg(make_env_path_arg())
        .arg(make_env_arg())
        .arg(make_arg_arg())
        .arg(make_api_url_arg())
        .arg(make_api_bind_url_arg())
        .arg(make_no_bind_arg())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(client::make_command())
        .subcommand(api::make_command())
        .get_matches()
}

fn make_arg_arg() -> Arg {
    Arg::new("config")
        .short('c')
        .long("config")
        .help("Path to config file")
        .action(ArgAction::Set)
}

fn make_env_path_arg() -> Arg {
    Arg::new("env-path")
        .short('e')
        .long("env-path")
        .help("Path to .env file")
        .action(ArgAction::Set)
}
fn make_env_arg() -> Arg {
    Arg::new("env")
        .short('E')
        .long("env")
        .help("Environment")
        .action(ArgAction::Set)
        .value_parser(value_parser!(Environment))
}

fn make_api_url_arg() -> Arg {
    Arg::new("api-url")
        .short('s')
        .long("api-url")
        .help("Api url to optionally bind on (see api-bind-url and no-bind)")
        .action(ArgAction::Set)
}

fn make_api_bind_url_arg() -> Arg {
    Arg::new("api-bind-url")
        .short('b')
        .long("api-bind-url")
        .help("Optional different url to bind on.")
        .action(ArgAction::Set)
}
fn make_no_bind_arg() -> Arg {
    Arg::new("no-bind")
        .short('n')
        .long("no-bind")
        .help("Do not attempt to run server in background")
        .action(ArgAction::SetTrue)
}
