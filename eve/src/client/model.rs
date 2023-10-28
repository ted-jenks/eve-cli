use crate::error::error::EveError;
use std::str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Model {
    Gpt35Turbo,
}

impl Model {
    pub(crate) fn from_str(s: &str) -> Result<Model, EveError> {
        match s {
            "gpt-3.5-turbo-instruct" => Ok(Model::Gpt35Turbo),
            _ => Err(EveError::new(
                ("Invalid model varient configured. Fix ~.evecfg or run `eve config`. Valid varients: ".to_string() + Self::valid_models())
                    .as_str(),
            )),
        }
    }

    pub(crate) fn valid_models() -> &'static str {
        Model::Gpt35Turbo.to_string()
    }

    pub(crate) fn to_string(self) -> &'static str {
        match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo-instruct",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_from_str_valid() {
        let model_str = "gpt-3.5-turbo-instruct";
        let result = Model::from_str(model_str);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Model::Gpt35Turbo);
    }

    #[test]
    fn test_model_from_str_invalid() {
        let model_str = "invalid_model";
        let result = Model::from_str(model_str);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "\u{1b}[31mError\u{1b}[0m: Invalid model varient configured. Fix ~.evecfg or run `eve config`. Valid varients: gpt-3.5-turbo-instruct"
        );
    }

    #[test]
    fn test_valid_models() {
        let expected_result = "gpt-3.5-turbo-instruct";
        assert_eq!(Model::valid_models(), expected_result);
    }

    #[test]
    fn test_model_to_string() {
        let model = Model::Gpt35Turbo;
        let expected_result = "gpt-3.5-turbo-instruct";
        assert_eq!(model.to_string(), expected_result);
    }
}
