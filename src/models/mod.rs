mod config;
mod mail;
mod imap;
mod matrix;

pub use config::Configuration;
pub use imap::ImapServer;
pub use mail::Mail;
pub use matrix::MatrixClient;
