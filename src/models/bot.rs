pub struct Bot;
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

impl Bot{
    pub async fn response(command: &str) -> Option<String>{
        let command = command.to_lowercase();
        if command == "!hola"{
            Some("Coca Cola".to_string())
        }else if command == "!hora"{
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Some(current_time.to_string())
        }else if command == "!tiempo"{
            let query = json!({
                "lang": "es",
                "format": 3
            });
            let client = reqwest::Client::new();
            client.get("https://wttr.in/silla")
                .query(&query)
                .send()
                .await
                .ok()?
                .text()
                .await
                .ok()
        }else{
            None
        }
    }
}

