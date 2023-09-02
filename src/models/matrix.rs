use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use urlencoding::encode;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{info, debug};
use reqwest::{Client, header::{HeaderMap, HeaderValue,
    HeaderName}};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatrixClient{
    url: String,
    token: String,
    room: String
}

impl MatrixClient {
    pub async fn post(&self, message: &str) -> Result<String, String>{
        info!("post_with_matrix");
        debug!("Post with matrix: {}", message);
        let now = SystemTime::now();
        let ts = now.duration_since(UNIX_EPOCH).expect("Time went backwrds").as_secs();
        let url = format!(
            "https://{}/_matrix/client/v3/rooms/{}:{}/send/m.room.message/{}",
            self.url,
            encode(self.room.as_str()),
            self.url,
            ts
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

    async fn _put(url: &str, header_map: HeaderMap, body: &Value) -> Result<String, String>{
        let client = Client::builder()
            .default_headers(header_map)
            .build()
            .unwrap();
        let content = serde_json::to_string(body).unwrap();
        client.put(url).body(content).send()
            .await
            .map_err(|err| err.to_string())?
            .text()
            .await
            .map_err(|err| err.to_string())
    }
}
