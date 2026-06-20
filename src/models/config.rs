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

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_yaml() -> &'static str {
        r#"
pull_time: 60
imap_server:
  host: imap.example.com
  port: 993
  user: test@example.com
  password: secret
matrix_client:
  protocol: https
  server: matrix.example.com
  token: test-token
  email_room: "!email:example.com"
  chat_room: "!chat:example.com"
  sender: testuser
  timeout: 30000
openai_client:
  protocol: https
  server: api.openai.com
  api_key: test-key
  model: gpt-4
  temperature: 0.7
  prompts:
    historia:
      prompt: Eres un historiador
      messages: []
"#
    }

    #[test]
    fn configuration_new_valid_yaml() {
        let config = Configuration::new(valid_yaml());
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.get_pull_time(), 60);
    }

    #[test]
    fn configuration_new_invalid_yaml() {
        let config = Configuration::new("invalid: [yaml: broken");
        assert!(config.is_err());
    }

    #[test]
    fn configuration_new_empty_yaml() {
        let config = Configuration::new("");
        assert!(config.is_err());
    }

    #[test]
    fn get_imap_server_returns_reference() {
        let config = Configuration::new(valid_yaml()).unwrap();
        let _imap = config.get_imap_server();
    }

    #[test]
    fn get_matrix_client_returns_reference() {
        let config = Configuration::new(valid_yaml()).unwrap();
        let matrix = config.get_matrix_client();
        assert_eq!(matrix.email_room, "!email:example.com");
        assert_eq!(matrix.chat_room, "!chat:example.com");
    }

    #[test]
    fn get_openai_client_returns_clone() {
        let config = Configuration::new(valid_yaml()).unwrap();
        let client = config.get_openai_client();
        assert_eq!(client.prompts.len(), 1);
        assert!(client.prompts.contains_key("historia"));
    }
}
