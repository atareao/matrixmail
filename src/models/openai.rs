use serde::{Deserialize, Serialize};
use super::BotError;
use std::collections::hash_map::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    prompt: String,
    messages: Vec<ChatMessage>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAIClient {
    protocol: String,
    server: String,
    api_key: String,
    model: String,
    temperature: f32,
    pub prompts: HashMap<String, Prompt>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ChatMessage,
}

impl OpenAIClient {
    pub async fn send_message(&mut self, name: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let prompt_data = self.prompts.get_mut(name).ok_or("Prompt not found")?;
        prompt_data.messages.push(ChatMessage {
            role: "user".to_string(),
            content: message.to_string(),
        });
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: prompt_data.messages.clone(),
            temperature: self.temperature,
        };
        let client = reqwest::Client::new();
        let response = client.post(format!("{}://{}/v1/chat/completions", self.protocol, self.server))
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;
        if response.status().is_success() {
            let response_body: OpenAIResponse = response.json().await?;
            let response = response_body.choices[0].message.content.clone();
            prompt_data.messages.push(ChatMessage {
                role: "assistant".to_string(),
                content: response.clone(),
            });
            Ok(response.clone())
        } else {
            Err(BotError::get_error("Request failed"))
        }
    }
    pub fn clear_messages(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let prompt_data = self.prompts.get_mut(name).ok_or("Prompt not found")?;
        prompt_data.messages.clear();
        Ok(())
    }
}
