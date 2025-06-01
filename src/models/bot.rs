pub struct Bot;
use std::time::{SystemTime, UNIX_EPOCH};
use super::OpenAIClient;

use serde_json::json;

impl Bot{
    pub async fn response(command: &str, openai_client: &mut OpenAIClient) -> Option<String>{
        let command = command.to_lowercase();
        if command == "!?"{
            Some("**Comandos disponibles**:\n\
                - !? - Muestra esta ayuda\n\
                - !c - Limpia el historial de mensajes\n\
                - !h - Hora actual\n\
                - !h <pregunta> - Consulta a OpenAI\n\
                - !t - Tiempo en Silla".to_string()
            )
        }else if command == "!c"{
            openai_client.clear_messages();
            Some("Historial de mensajes limpiado".to_string())
        }else if command == "!h"{
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Some(current_time.to_string())
        }else if command.starts_with("!h ") {
            let pregunta = command.trim_start_matches("!h ").trim();
            match openai_client.send_message(pregunta).await {
                Ok(response) => Some(response),
                Err(e) => Some(format!("**Error** consultando OpenAI: {e}")),
            }
        }else if command == "!t"{
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

