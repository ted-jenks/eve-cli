use std::error::Error;
use std::fmt;

pub struct EveError {
    message: String,
}

impl EveError {
    pub fn new(message: &str) -> EveError {
        EveError {
            message: message.to_string(),
        }
    }
}

impl Error for EveError {}

impl fmt::Display for EveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl fmt::Debug for EveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}
