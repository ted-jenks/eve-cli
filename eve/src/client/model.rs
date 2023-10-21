use crate::error::error::EveError;
use std::str;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Model {
    Gpt35Turbo,
}

impl Model {
    pub(crate) fn from_str(s: &str) -> Result<Model, EveError> {
        let valid_models: &str = Model::Gpt35Turbo.to_string();
        match s {
            "gpt-3.5-turbo-instruct" => Ok(Model::Gpt35Turbo),
            _ => Err(EveError::new(
                ("Invalid model varient configured. Fix ~.evecfg or run `eve config`. Valid varients: ".to_string() + valid_models)
                    .as_str(),
            )),
        }
    }

    pub(crate) fn to_string(self) -> &'static str {
        match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo-instruct",
        }
    }
}
