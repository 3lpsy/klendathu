use anyhow::Result;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use serde::Deserialize;

#[derive(Copy, Clone, Default, Debug, Deserialize)]
pub enum Environment {
    Testing,
    #[default]
    Local,
    Development,
    Production,
}

impl ValueEnum for Environment {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Environment::Testing,
            Environment::Local,
            Environment::Development,
            Environment::Production,
        ]
    }
    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(match self {
            Environment::Testing => PossibleValue::new("local").help("Run in testing mode"),
            Environment::Local => PossibleValue::new("local").help("Run in local development mode"),
            Environment::Development => {
                PossibleValue::new("local").help("Run in development deployment mode")
            }
            Environment::Production => {
                PossibleValue::new("local").help("Run in development production mode")
            }
        })
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for v in Self::value_variants() {
            if v.to_possible_value().unwrap().matches(s, false) {
                return Ok(*v);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}
