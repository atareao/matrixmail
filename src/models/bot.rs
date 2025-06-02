pub struct Bot;
use std::time::{SystemTime, UNIX_EPOCH};
use super::OpenAIClient;

use serde_json::json;

impl Bot{
    pub async fn response(command: &str, openai_client: &mut OpenAIClient, names: &Vec<String>) -> Option<String>{
        let command = command.to_lowercase();
        if command == "!?"{
            let mut commands = "**Comandos disponibles**:\n\
                - !? - Muestra esta ayuda\n\
                - !c <prompt> - Limpia el historial de mensajes del <prompt>\n\
                - !h - Hora actual\n\
                - !t - Tiempo en Silla\n".to_string();
            for name in names {
                commands.push_str(&format!("- !{} <pregunta> - Consulta al prompt `{}`\n", name.chars().next().unwrap(), name));
            }
            Some(commands.to_string())
        }else if command == "!c "{
            let prompt_name = command.trim_start_matches("!c ").trim();
            match openai_client.clear_messages(prompt_name){
                Ok(_) => Some(format!("**Historial de mensajes del prompt `{}` limpiado**", prompt_name)),
                Err(e) => Some(format!("**Error** limpiando el historial de mensajes: {e}")),
            }
        }else if command == "!h"{
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs_f64();
            Some(current_time.to_string())
        }else if command.starts_with("!h ") {
            let pregunta = command.trim_start_matches("!h ").trim();
            match openai_client.send_message("historia", pregunta).await {
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
            for name in names {
                let key = name.to_lowercase().chars().next().unwrap();
                if command.starts_with(&format!("!{} ", key)) {
                    let pregunta = command.trim_start_matches(&format!("!{} ", key)).trim();
                    match openai_client.send_message(name, pregunta).await {
                        Ok(response) => return Some(response.clone()),
                        Err(e) => return Some(format!("**Error** consultando OpenAI: {e}")),
                    }
                }
            }
            None
        }
    }
}

