mod models;

use std::sync::Arc;
use std::{process, time};
use tracing_subscriber::{
    fmt,
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use models::{
    Configuration,
    MatrixClient,
    Bot,
};
use serde_json::Value;
use tracing::{info, debug, error};

#[tokio::main]
async fn main(){
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
    info!("Start");
    tokio::spawn( async {
        let configuration = match Configuration::read().await{
            Ok(configuration) => Arc::new(configuration),
            Err(e) =>{
                println!("Error. Can not read configuration: {}", e);
                process::exit(0);
            }
        };
        debug!("Configuration: {:?}", &configuration);
        let sleep_time: u64 = configuration.get_pull_time().into();
        let duration = time::Duration::from_secs(sleep_time);
        let imap_server = configuration.get_imap_server();
        let matrix_client = configuration.get_matrix_client();
        loop{
            debug!("==== START LOOP ====");
            match imap_server.get_unread_mails().await{
                Ok(mails) => {
                    for mail in mails.as_slice(){
                        debug!("{}", &mail);
                        match matrix_client.post(&mail.to_string()).await{
                            Ok(response) => debug!(response),
                            Err(error_message) => error!(error_message)

                        }
                    }
                    if mails.is_empty(){
                        debug!("Not found any mail");
                    }
                },
                Err(message) => error!("{}", message),
            };
            debug!("==== END LOOP ====");
            tokio::time::sleep(duration).await;
        }
    });
    let mut configuration2 = match Configuration::read().await{
        Ok(configuration) => configuration,
        Err(e) =>{
            println!("Error. Can not read configuration: {}", e);
            process::exit(0);
        }
    };
    loop {
        debug!("Leo");
        match configuration2.matrix_client.sync().await{
            Ok(response) => {
                debug!("Response: {:?}", response);
                if let Some(response) = response {
                    match configuration2.save().await{
                        Ok(()) => {
                            debug!("Configuration saved");
                            debug!("Configuration: {:?}", &configuration2);
                        },
                        Err(e) => error!("Cant save configuration: {}", e),
                    };
                    if let Some(command) = process_response(&response, &configuration2.matrix_client){
                        debug!(command);
                        if let Some(message) = Bot::response(&command).await {
                            match &configuration2.matrix_client.post(&message).await{
                                Ok(response) => debug!("Response: {}", response),
                                Err(e) => error!("Error: {}", e),
                            }
                        }
                    }
                }
            },
            Err(error) => {
                error!("Can not sync: {:#}", error);
                let mut next_err = error.source();
                while next_err.is_some(){
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

fn process_response(value: &Value, matrix_client: &MatrixClient) -> Option<String>{
    let sender = matrix_client.get_sender_id();

    if let Some(rooms) = value.get("rooms")
        .and_then(|rooms| rooms.get("join"))
        .and_then(|join| join.as_object()) 
    {
        for room in rooms.values() {
            if let Some(timeline) = room.get("timeline")
                .and_then(|timeline| timeline.get("events"))
                .and_then(|events| events.as_array()) 
            {
                for event in timeline {
                    if let Some(current_sender) = event.get("sender")
                            .and_then(|sender| sender.as_str()) {
                        debug!("Sender: {}", current_sender);
                        if sender == current_sender {
                            return None;
                        }
                    }
                    if let Some(body) = event.get("content")
                        .and_then(|content| {
                            debug!("Evento: {:?}", content);
                            content.get("msgtype")
                                .and_then(|msgtype| msgtype.as_str())
                                .filter(|&msgtype| msgtype == "m.text")
                                .and_then(|_| content.get("body"))
                                .and_then(|body| body.as_str())
                        }) 
                    {
                        return Some(body.to_string());
                    }
                }
            }
        }
    }
    None
}
