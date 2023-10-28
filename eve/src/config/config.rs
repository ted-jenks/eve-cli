use crate::client::model::Model;
use crate::error::error::EveError;
use std::fs;
use std::path::PathBuf;

const MODEL: &str = "MODEL";
const API_KEY: &str = "API_KEY";

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_config_from_file_valid() {
        let model_str = "gpt-3.5-turbo-instruct";
        let api_key = "test_api_key";

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "MODEL={}", model_str).unwrap();
        writeln!(file, "API_KEY={}", api_key).unwrap();

        let result = Config::from_file(file.path().to_path_buf());
        assert!(result.is_ok());

        let config = result.unwrap();
        assert_eq!(config.model(), Model::Gpt35Turbo);
        assert_eq!(config.api_key(), api_key);
    }

    #[test]
    fn test_config_from_file_invalid_model() {
        let model_str = "invalid_model";
        let api_key = "test_api_key";

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "MODEL={}", model_str).unwrap();
        writeln!(file, "API_KEY={}", api_key).unwrap();

        let result = Config::from_file(file.path().to_path_buf());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("\u{1b}[31mError\u{1b}[0m: Invalid model in config: {}", format!("\u{1b}[31mError\u{1b}[0m: Invalid model varient configured. Fix ~.evecfg or run `eve config`. Valid varients: {}", Model::valid_models()))
        );
    }

    #[test]
    fn test_config_from_file_missing_model() {
        let api_key = "test_api_key";

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "API_KEY={}", api_key).unwrap();

        let result = Config::from_file(file.path().to_path_buf());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "\u{1b}[31mError\u{1b}[0m: Model not found in the config"
        );
    }

    #[test]
    fn test_config_from_file_missing_api_key() {
        let model_str = "gpt-3.5-turbo-instruct";

        let mut file = NamedTempFile::new().unwrap();
        writeln!(file, "MODEL={}", model_str).unwrap();

        let result = Config::from_file(file.path().to_path_buf());
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "\u{1b}[31mError\u{1b}[0m: API key not found in the config"
        );
    }
}
