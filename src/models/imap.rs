use async_native_tls::TlsConnector;
use async_std::net::TcpStream;
use async_imap::Client;
use futures::TryStreamExt;
use serde::{Serialize, Deserialize};
use mail_parser::Message;
use super::mail::HeaderMail;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImapServer{
    host: String,
    port: u16,
    user: String,
    password: String
}

impl ImapServer{
    #[allow(unused)]
    pub async fn read_mail(&self, message_id: &str) -> Result<String, String>{
        let user = &self.user;
        let password = &self.password;
        let server = self.host.as_str();
        let imap_addr = (server, self.port);
        let tcp_stream = TcpStream::connect(imap_addr).await.map_err(|err|
            err.to_string())?;
        let tls = TlsConnector::new();
        let tls_stream = tls.connect(server, tcp_stream).await.map_err(|err|
            err.to_string())?;

        let client = Client::new(tls_stream);
        //let client = async_imap::connect( (self.host, self.port), self.host, tls).await.unwrap();
        let mut imap_session = client
            .login(user, password)
            .await
            .map_err(|e| e.0).unwrap();
        imap_session.select("INBOX").await.unwrap();
        let messages_stream = imap_session.fetch(message_id, "RFC822").await.unwrap();
        let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
        let message = if let Some(m) = messages.first(){
            m
        }else{
            return Ok("".to_string());
        };
        let body = message.body().unwrap_or(b"");
        let body = std::str::from_utf8(body).unwrap_or("").to_string();
        imap_session.logout().await.unwrap();
        Ok(body)
    }

    pub async fn get_unread_mails(&self) -> Result<Vec<HeaderMail>, String>{
        let user = &self.user;
        let password = &self.password;
        let server = self.host.as_str();
        let mut result:Vec<HeaderMail> = Vec::new();
        let imap_addr = (self.host.as_str(), self.port);
        let tcp_stream = TcpStream::connect(imap_addr).await.map_err(|err|
            err.to_string())?;
        let tls = TlsConnector::new();
        let tls_stream = tls.connect(server, tcp_stream).await.map_err(|err|
            err.to_string())?;

        let client = Client::new(tls_stream);
        //let client = async_imap::connect( (self.host, self.port), self.host, tls).await.unwrap();
        let mut imap_session = client
            .login(user, password)
            .await
            .map_err(|e| e.0).unwrap();
        imap_session.select("INBOX").await.unwrap();
        let mut new_items = imap_session.search("NOT SEEN").await.unwrap();
        if let Some(identificador) = new_items.drain().next(){
            let messages_stream = imap_session.fetch(identificador.to_string(), "RFC822").await.unwrap();
            let messages: Vec<_> = messages_stream.try_collect().await.unwrap();
            for message in messages {
                let id = message.message;
                let body = message.body().unwrap();
                match Message::parse(body){
                    Some(content) => result.push(HeaderMail::new(id, &content)),
                    None => println!("{:?}", body),
                }
            }
        }
        Ok(result)
    }
}
