mod config;
mod mail;
mod imap;
mod matrix;
mod bot;
mod openai;
mod error;

pub use config::Configuration;
pub use imap::ImapServer;
pub use matrix::MatrixClient;
pub use openai::OpenAIClient;
pub use bot::Bot;
pub use error::BotError;

pub type CustomError = Box<dyn std::error::Error>;
