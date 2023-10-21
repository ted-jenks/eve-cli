use crate::client::model::Model;
use crate::error::error::EveError;
use std::fs;

const MODEL: &str = "MODEL";
const API_KEY: &str = "API_KEY";

pub struct Config {
    model: Option<Model>,
    api_key: Option<String>,
}

impl Config {
    fn new() -> Config {
        Config {
            model: None,
            api_key: None,
        }
    }

    pub fn from_file(filename: &str) -> Result<Config, EveError> {
        let contents = fs::read_to_string(filename).map_err(|_e| {
            EveError::new(
                "Could not locate config file. Set up your ~.evecfg file or run `eve config`.",
            )
        })?;

        let mut config = Config::new();

        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0] {
                    MODEL => config.model = Some(Model::from_str(parts[1])?),
                    API_KEY => config.api_key = Some(parts[1].to_string()),
                    _ => (),
                }
            }
        }

        Ok(config)
    }

    pub(crate) fn model(&self) -> Option<&Model> {
        self.model.as_ref()
    }

    pub(crate) fn api_key(&self) -> Option<&String> {
        self.api_key.as_ref()
    }
}
