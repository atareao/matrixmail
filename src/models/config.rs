use serde::{Serialize, Deserialize};
use tracing::{info, debug};
use super::CustomError;

use super::{ImapServer, MatrixClient, OpenAIClient};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    pull_time: u16,
    imap_server: ImapServer,
    pub matrix_client: MatrixClient,
    pub openai_client: OpenAIClient,
}


impl Configuration {
    pub fn new(content: &str) -> Result<Configuration, CustomError>{
        info!("new");
        debug!("Content: {}", content);
        Ok(serde_yaml::from_str(content)?)
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

    pub fn get_openai_client(&self) -> OpenAIClient{
        info!("get_openai_client");
        self.openai_client.clone()
    }

    pub async fn read() -> Result<Configuration, CustomError>{
        debug!("read");
        let content = tokio::fs::read_to_string("config.yml").await?;
        Self::new(&content)
    }

    pub async fn save(&self) -> Result<(), CustomError>{
       debug!("save");
       let content = serde_yaml::to_string(&self)?;
       Ok(tokio::fs::write("config.yml", content).await?)
    }
}
