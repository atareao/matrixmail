use std::fmt;

#[derive(Debug)]
pub struct BotError {
    msg: String,
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

impl std::error::Error for BotError {}

impl BotError {

    pub fn new(msg: &str) -> Self {
        BotError {
            msg: msg.to_string(),
        }
    }
    pub fn get_error(msg: &str) -> Box<dyn std::error::Error> {
        Box::new(BotError::new(msg))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bot_error_new_creates_error_with_message() {
        let err = BotError::new("test error");
        assert_eq!(format!("{}", err), "Error: test error");
    }

    #[test]
    fn bot_error_get_error_returns_boxed_error() {
        let err = BotError::get_error("something went wrong");
        assert_eq!(format!("{}", err), "Error: something went wrong");
    }

    #[test]
    fn bot_error_implements_std_error() {
        let err = BotError::new("test");
        let trait_object: &dyn std::error::Error = &err;
        assert_eq!(trait_object.to_string(), "Error: test");
    }
}
