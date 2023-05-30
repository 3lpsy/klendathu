use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{env, fs};

use crate::config::Environment;
use anyhow::Result;
use clap::ArgMatches;
use directories::ProjectDirs;
use serde::Deserialize;
use toml;

use crate::{constants, utils};

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub environment: Option<Environment>,
    pub api: Option<ConfigApi>,
    pub cli: Option<ConfigCli>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Default, Deserialize)]
pub struct ConfigApi {
    pub db_url: Option<String>,
    pub api_bind_url: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[allow(dead_code)]
pub struct ConfigCli {
    pub connect_url: Option<String>,
    pub api_url: Option<String>,
}

impl Config {
    pub fn new(environment: Environment, cli: Option<ConfigCli>, api: Option<ConfigApi>) -> Self {
        Self {
            environment: Some(environment),
            cli,
            api,
        }
    }

    pub fn from_matches(matches: &ArgMatches) -> Result<Self> {
        // first determine the mode

        match matches.get_one::<String>("env-path") {
            Some(env_path) => {
                utils::load_env_file(env_path);
            }
            _ => {}
        };

        // prioritize CLI if it's set
        let env: Option<Environment> = match matches.get_one::<Environment>("env") {
            Some(from_cli) => Some(*from_cli),
            _ => match std::env::var(constants::ENV_MODE_KEY) {
                // fallback to env if not set
                Ok(from_env) => Some(Environment::from_str(&from_env).map_err(anyhow::Error::msg)?),
                Err(_) => None,
            },
        };

        // next we need to determine where the config file is.

        // prioritize cli
        let config_path: Option<PathBuf> = match matches.get_one::<String>("config") {
            Some(from_cli) => {
                let from_cli_path = Path::new(from_cli);
                let canon = from_cli_path.canonicalize()?;
                if !from_cli_path.exists() {
                    return Err(anyhow::Error::msg(format!(
                        "Config path {} does not exist.",
                        canon.to_string_lossy()
                    )));
                }
                Some(canon)
            }
            _ => match std::env::var(constants::ENV_KLENDATHU_CONFIG_PATH_KEY) {
                // fallback to env
                Ok(from_env) => {
                    let from_env_path = Path::new(&from_env);
                    let canon = from_env_path.canonicalize()?;
                    Some(canon)
                }
                Err(e) => {
                    // if not set, first look at pwd for the file
                    let mut pwd_config =
                        env::current_dir().expect("Could not get current working directory");
                    pwd_config.push(PathBuf::from(constants::DEFAULT_CONFIG_FILE_NAME));
                    match pwd_config.exists() {
                        true => Some(pwd_config),
                        false => {
                            let pd = ProjectDirs::from("io", "elpsy", "klendathu")
                                .expect("Unable to get project directory from xdg");
                            match pd.config_dir().exists() {
                                true => {
                                    let mut default_xdg_config =
                                        pd.clone().config_dir().to_path_buf();
                                    default_xdg_config
                                        .push(PathBuf::from(constants::DEFAULT_CONFIG_FILE_NAME));
                                    match default_xdg_config.exists() {
                                        true => Some(default_xdg_config),
                                        false => None,
                                    }
                                }
                                false => None,
                            }
                        }
                    }
                }
            },
        };
        // if it exists, it needs to be parseable
        let mut config = match config_path {
            Some(config_path_val) => {
                let toml_config_content = fs::read_to_string(config_path_val)?;
                let parsed: Config = toml::from_str(&toml_config_content)?;
                // want to overrwrite env if passed from cli
                let environment = match env {
                    Some(env_val) => env_val,
                    None => Environment::default(),
                };
                Config::new(environment, parsed.cli, parsed.api)
            }
            None => {
                // defaults to local
                let environment = env.unwrap_or(Environment::default());
                Config::new(
                    environment,
                    Some(ConfigCli::default()),
                    Some(ConfigApi::default()),
                )
            }
        };
        let api_url_cli = matches.get_one::<String>("api-url");
        let api_bind_url_cli = matches.get_one::<String>("api-bind-url");

        if api_url_cli.is_none() && api_bind_url_cli.is_none() {
            if let Some(api) = &mut config.api {
                api.api_bind_url = Some(constants::DEFAULT_API_URL.to_string());
            }
            if let Some(cli) = &mut config.cli {
                cli.api_url = Some(constants::DEFAULT_API_URL.to_string());
            }
        }
        // now we need to figure out where the api is / should be.
        // two main urls to parse the normal api-url and api-bind-url.

        Ok(config)
    }
}
