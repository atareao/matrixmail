mod config;
mod mail;
mod imap;
mod matrix;
mod bot;

pub use config::Configuration;
pub use imap::ImapServer;
pub use mail::Mail;
pub use matrix::MatrixClient;
pub use bot::Bot;

pub type CustomError = Box<dyn std::error::Error>;
