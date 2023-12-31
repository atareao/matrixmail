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
        let duration = time::Duration::from_secs(
            configuration.get_pull_time().into()); // 5 minutes
        let imap_server = configuration.get_imap_server();
        let matrix_client = configuration.get_matrix_client();
        loop{
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
    if let Some(rooms) = value.get("rooms"){
        if let Some(join) = rooms.get("join"){
            for room_id in join.as_object().unwrap().keys(){
                if let Some(room) = join.get(room_id){
                    if let Some(timeline) = room.get("timeline"){
                        if let Some(events) = timeline.get("events"){
                            for event in events.as_array().unwrap(){
                                if let Some(current_sender) = event.get("sender"){
                                    debug!("Sender: {}", current_sender);
                                    if sender == current_sender.as_str().unwrap(){
                                        return None;
                                    }
                                }
                                if let Some(content) = event.get("content"){
                                    debug!("Evento: {:?}", content);
                                    let msgtype = content.get("msgtype")
                                        .unwrap()
                                        .as_str()
                                        .unwrap();
                                    if msgtype == "m.text"{
                                        let body = content.get("body")
                                            .unwrap()
                                            .as_str()
                                            .unwrap();
                                        debug!("CONTENIDO: {}", body);
                                        if body.starts_with('!'){
                                            return Some(body.to_string());
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
