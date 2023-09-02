use serde::{Serialize, Deserialize};
use serde_yaml::Error;
use tracing::{info, debug};

use super::{ImapServer, MatrixClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    log_level: String,
    pull_time: u16,
    imap_server: ImapServer,
    matrix_client: MatrixClient,
}


impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, Error>{
        info!("new");
        debug!("Content: {}", content);
        serde_yaml::from_str(content)
    }

    pub fn get_log_level(&self) -> &str{
        info!("get_log_level");
        &self.log_level
    }

    pub fn get_pull_time(&self) -> u16{
        info!("get_pull_time");
        self.pull_time
    }

    pub fn get_imap_server(&self) -> &ImapServer{
        info!("get_imap_server");
        &self.imap_server
    }

    pub fn get_matrix_client(&self) -> &MatrixClient{
        info!("get_matrix_client");
        &self.matrix_client
    }
}
