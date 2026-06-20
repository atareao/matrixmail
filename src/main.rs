mod models;

use models::{Bot, Configuration, MatrixClient};
use serde_json::Value;
use std::sync::Arc;
use std::{process, time};
use tracing::{debug, error, info};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    info!("Start");
    tokio::spawn(async {
        let configuration = match Configuration::read().await {
            Ok(configuration) => Arc::new(configuration),
            Err(e) => {
                println!("Error. Can not read configuration: {}", e);
                process::exit(0);
            }
        };
        debug!("Configuration: {:?}", &configuration);
        let sleep_time: u64 = configuration.get_pull_time().into();
        let duration = time::Duration::from_secs(sleep_time);
        let imap_server = configuration.get_imap_server();
        let matrix_client = configuration.get_matrix_client();
        loop {
            debug!("==== START LOOP ====");
            match imap_server.get_unread_mails().await {
                Ok(mails) => {
                    for mail in mails.as_slice() {
                        debug!("{}", &mail);
                        match matrix_client.post_to_email_room(&mail.to_string()).await {
                            Ok(response) => debug!(response),
                            Err(error_message) => error!(error_message),
                        }
                    }
                    if mails.is_empty() {
                        debug!("Not found any mail");
                    }
                }
                Err(message) => error!("{}", message),
            };
            debug!("==== END LOOP ====");
            tokio::time::sleep(duration).await;
        }
    });
    let mut configuration2 = match Configuration::read().await {
        Ok(configuration) => configuration,
        Err(e) => {
            println!("Error. Can not read configuration: {}", e);
            process::exit(0);
        }
    };
    let mut openai_client = configuration2.get_openai_client();
    let keys = openai_client.prompts.keys().cloned().collect::<Vec<_>>();
    loop {
        debug!("===== Leo =====");
        match configuration2.matrix_client.sync().await {
            Ok(response) => {
                debug!("Response: {:?}", response);
                if let Some(response) = response {
                    match configuration2.save().await {
                        Ok(()) => {
                            debug!("Configuration saved");
                            debug!("Configuration: {:?}", &configuration2);
                        }
                        Err(e) => error!("Cant save configuration: {}", e),
                    };
                    if let Some((room, command)) =
                        process_response(&response, &configuration2.matrix_client)
                    {
                        debug!(command);
                        if let Some(message) = Bot::response(&command, &mut openai_client, &keys).await {
                            if configuration2.matrix_client.chat_room == room {
                                match &configuration2
                                    .matrix_client
                                    .post_to_chat_room(&message)
                                    .await
                                {
                                    Ok(response) => debug!("Response: {}", response),
                                    Err(e) => error!("Error: {}", e),
                                }
                            }
                            if configuration2.matrix_client.email_room == room {
                                match &configuration2
                                    .matrix_client
                                    .post_to_email_room(&message)
                                    .await
                                {
                                    Ok(response) => debug!("Response: {}", response),
                                    Err(e) => error!("Error: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(error) => {
                error!("Can not sync: {:#}", error);
                let mut next_err = error.source();
                while next_err.is_some() {
                    error!("caused by: {:#}", next_err.unwrap());
                    next_err = next_err.unwrap().source();
                }
                let duration = time::Duration::from_secs(5);
                tokio::time::sleep(duration).await;
            }
        }
        tokio::time::sleep(time::Duration::from_secs(2)).await;
    }
}

fn process_response(value: &Value, matrix_client: &MatrixClient) -> Option<(String, String)> {
    let sender = matrix_client.get_sender_id();

    if let Some(rooms) = value
        .get("rooms")
        .and_then(|rooms| rooms.get("join"))
        .and_then(|selected_room| selected_room.as_object())
    {
        for (room, room_value) in rooms {
            let compare_room = room.split(':').next().unwrap();
            debug!("Compare room: {compare_room}");
            debug!(
                "=== {} <=> {} / {} / {} ===",
                matrix_client.chat_room, compare_room, room, room_value
            );
            if compare_room != matrix_client.email_room && compare_room != matrix_client.chat_room {
                return None;
            }
            if let Some(timeline) = room_value
                .get("timeline")
                .and_then(|timeline| timeline.get("events"))
                .and_then(|events| events.as_array())
            {
                for event in timeline {
                    if let Some(current_sender) =
                        event.get("sender").and_then(|sender| sender.as_str())
                    {
                        debug!("Sender: {}", current_sender);
                        if sender == current_sender {
                            return None;
                        }
                    }
                    if let Some(body) = event.get("content").and_then(|content| {
                        debug!("Evento: {:?}", content);
                        content
                            .get("msgtype")
                            .and_then(|msgtype| msgtype.as_str())
                            .filter(|&msgtype| msgtype == "m.text")
                            .and_then(|_| content.get("body"))
                            .and_then(|body| body.as_str())
                    }) {
                        return Some((compare_room.to_string(), body.to_string()));
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_matrix_client() -> MatrixClient {
        serde_json::from_value(json!({
            "protocol": "https",
            "server": "matrix.example.com",
            "token": "test-token",
            "email_room": "!email",
            "chat_room": "!chat",
            "sender": "testuser",
            "timeout": 30000,
            "since": null
        }))
        .unwrap()
    }

    #[test]
    fn process_response_no_rooms_returns_none() {
        let value = json!({});
        let client = create_matrix_client();
        assert!(process_response(&value, &client).is_none());
    }

    #[test]
    fn process_response_no_join_rooms_returns_none() {
        let value = json!({"rooms": {}});
        let client = create_matrix_client();
        assert!(process_response(&value, &client).is_none());
    }

    #[test]
    fn process_response_wrong_room_returns_none() {
        let value = json!({
            "rooms": {
                "join": {
                    "!other:example.com": {
                        "timeline": {
                            "events": [{
                                "sender": "@other:example.com",
                                "content": {
                                    "msgtype": "m.text",
                                    "body": "hello"
                                }
                            }]
                        }
                    }
                }
            }
        });
        let client = create_matrix_client();
        assert!(process_response(&value, &client).is_none());
    }

    #[test]
    fn process_response_own_message_returns_none() {
        let value = json!({
            "rooms": {
                "join": {
                    "!chat:example.com": {
                        "timeline": {
                            "events": [{
                                "sender": "@testuser:matrix.example.com",
                                "content": {
                                    "msgtype": "m.text",
                                    "body": "hello"
                                }
                            }]
                        }
                    }
                }
            }
        });
        let client = create_matrix_client();
        assert!(process_response(&value, &client).is_none());
    }

    #[test]
    fn process_response_valid_command_returns_command() {
        let value = json!({
            "rooms": {
                "join": {
                    "!chat:example.com": {
                        "timeline": {
                            "events": [{
                                "sender": "@otheruser:example.com",
                                "content": {
                                    "msgtype": "m.text",
                                    "body": "!?"
                                }
                            }]
                        }
                    }
                }
            }
        });
        let client = create_matrix_client();
        let result = process_response(&value, &client);
        assert!(result.is_some());
        let (room, command) = result.unwrap();
        assert_eq!(room, "!chat");
        assert_eq!(command, "!?");
    }

    #[test]
    fn process_response_non_text_message_ignored() {
        let value = json!({
            "rooms": {
                "join": {
                    "!chat:example.com": {
                        "timeline": {
                            "events": [{
                                "sender": "@otheruser:example.com",
                                "content": {
                                    "msgtype": "m.image",
                                    "body": "photo.png",
                                    "url": "mxc://..."
                                }
                            }]
                        }
                    }
                }
            }
        });
        let client = create_matrix_client();
        assert!(process_response(&value, &client).is_none());
    }

    #[test]
    fn process_response_email_room_works() {
        let value = json!({
            "rooms": {
                "join": {
                    "!email:example.com": {
                        "timeline": {
                            "events": [{
                                "sender": "@otheruser:example.com",
                                "content": {
                                    "msgtype": "m.text",
                                    "body": "test email command"
                                }
                            }]
                        }
                    }
                }
            }
        });
        let client = create_matrix_client();
        let result = process_response(&value, &client);
        assert!(result.is_some());
        let (room, command) = result.unwrap();
        assert_eq!(room, "!email");
        assert_eq!(command, "test email command");
    }
}
