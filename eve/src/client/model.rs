use crate::error::error::EveError;
use std::str;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Model {
    Gpt35Turbo,
}

impl Model {
    pub fn from_str(s: &str) -> Result<Model, EveError> {
        match s {
            "gpt-3.5-turbo" => Ok(Model::Gpt35Turbo),
            _ => Err(EveError::new("Invalid model varient configured")),
        }
    }

    pub fn to_string(self) -> &'static str {
        match self {
            Model::Gpt35Turbo => "gpt-3.5-turbo",
        }
    }
}
