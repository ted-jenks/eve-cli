use crate::client::model::Model;
use crate::error::error::EveError;
use std::fs;
use std::path::PathBuf;

const MODEL: &str = "MODEL";
const API_KEY: &str = "API_KEY";

pub(crate) struct Config {
    model: Model,
    api_key: String,
}

impl Config {
    fn new(model: Model, api_key: String) -> Config {
        Config { model, api_key }
    }

    pub(crate) fn from_file(filename: PathBuf) -> Result<Config, EveError> {
        let contents = fs::read_to_string(filename).map_err(|_e| {
            EveError::new(
                "Could not locate config file. Set up your ~/.evecfg file or run `eve config`.",
            )
        })?;

        let mut model: Option<Model> = None;
        let mut api_key: Option<String> = None;

        for line in contents.lines() {
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() == 2 {
                match parts[0] {
                    MODEL => {
                        let parsed_model = Model::from_str(parts[1]).map_err(|e| {
                            EveError::new(format!("Invalid model in config: {}", e).as_str())
                        })?;
                        model = Some(parsed_model);
                    }
                    API_KEY => {
                        api_key = Some(parts[1].to_string());
                    }
                    _ => (),
                }
            }
        }

        let model = model.ok_or_else(|| EveError::new("Model not found in the config"))?;
        let api_key = api_key.ok_or_else(|| EveError::new("API key not found in the config"))?;

        Ok(Config::new(model, api_key))
    }

    pub(crate) fn model(&self) -> Model {
        self.model
    }

    pub(crate) fn api_key(&self) -> &str {
        self.api_key.as_ref()
    }
}
