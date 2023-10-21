use std::error::Error;
use std::fmt;

pub(crate) struct EveError {
    message: String,
}

impl EveError {
    pub(crate) fn new(message: &str) -> EveError {
        EveError {
            message: message.to_string(),
        }
    }
}

impl Error for EveError {}

impl fmt::Display for EveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[31mError\x1b[0m: {}", self.message)
    }
}

impl fmt::Debug for EveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\x1b[31mError\x1b[0m: {}", self.message)
    }
}
