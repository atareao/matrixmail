pub struct Bot;
use std::time::{SystemTime, UNIX_EPOCH};
use super::{BotError, OpenAIClient};

use serde_json::json;

impl Bot{
    pub async fn response(command: &str, openai_client: &mut OpenAIClient) -> Option<String>{
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
        }else if command.starts_with("!historia") {
            let pregunta = command.trim_start_matches("!historia").trim();
            match openai_client.send_message(pregunta).await {
                Ok(respuesta) => Some(respuesta),
                Err(e) => Some(format!("Error consultando OpenAI: {e}")),
            }
        }else if command == "!clean"{
            openai_client.clear_messages();
            Some("Historial de mensajes limpiado".to_string())
        }else{
            None
        }
    }
}

