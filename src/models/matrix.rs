use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use urlencoding::encode;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug};
use reqwest::{Client, header::{HeaderMap, HeaderValue,
    HeaderName}};

use super::CustomError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixClient{
    protocol: String,
    server: String,
    token: String,
    room: String,
    sender: String,
    timeout: u64,
    #[serde(default = "get_default_since")]
    since: Option<String>,
}

fn get_default_since() -> Option<String>{
    None
}

impl MatrixClient {

    pub async fn sync(&mut self) -> Result<Value, CustomError>{
        debug!("sync");
        let url = format!("{}://{}/_matrix/client/v3/\
            sync", self.protocol, self.server);
        debug!("url: {}", url);
        let query = json!({
            "since": self.since,
            "timeout": self.timeout,
        });
        debug!("query: {:?}", query);
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let response = client.get(url)
            .query(&query)
            .send()
            .await?
            .json::<Value>()
            .await?;
        debug!("Response: {:?}", response);
        if let Some(value) = response.get("next_batch"){
            if let Some(next_batch) = value.as_str(){
                self.set_since(Some(next_batch.to_string()));
            }
        }
        Ok(response)
    }
    pub async fn post(&self, message: &str) -> Result<String, CustomError>{
        info!("post_with_matrix");
        debug!("Post with matrix: {}", message);
        let url = format!(
            "{}://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}",
            self.protocol,
            self.server,
            encode(self.room.as_str()),
            self.server,
            Self::ts(),
        );
        let body = json!({
            "msgtype": "m.text",
            "body": message,
        });
        let mut header_map = HeaderMap::new();
        header_map.insert(HeaderName::from_str("Content-type").unwrap(),
                          HeaderValue::from_str("application/json").unwrap());
        header_map.append(HeaderName::from_str("Authorization").unwrap(),
                          HeaderValue::from_str(&format!("Bearer {}", self.token)).unwrap());
        Self::_put(&url, header_map, &body)
            .await
    }

    async fn _put(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, CustomError>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        Ok(client.put(url).body(content).send()
            .await?
            .text()
            .await?)
    }

    pub fn set_since(&mut self, since: Option<String>){
        self.since = since;
    }

    pub fn get_sender_id(&self) -> String{
        format!("@{}:{}",
            self.sender,
            self.server
        )
    }

    fn ts() -> f64{
        debug!("ts");
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap() .as_secs_f64()
    }
}
