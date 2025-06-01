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
