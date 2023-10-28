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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_eve_error() {
        let error_message = "Test Error Message";
        let eve_error = EveError::new(error_message);
        assert_eq!(eve_error.message, error_message);
    }

    #[test]
    fn test_display_formatting() {
        let error_message = "Test Error Message";
        let eve_error = EveError::new(error_message);
        let formatted_message = format!("{}", eve_error);
        assert_eq!(
            formatted_message,
            format!("\x1b[31mError\x1b[0m: {}", error_message)
        );
    }

    #[test]
    fn test_debug_formatting() {
        let error_message = "Test Error Message";
        let eve_error = EveError::new(error_message);
        let formatted_debug_message = format!("{:?}", eve_error);
        assert_eq!(
            formatted_debug_message,
            format!("\x1b[31mError\x1b[0m: {}", error_message)
        );
    }
}
