mod models;

use tokio;
use std::{process, str::FromStr, time};
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
};
use models::Configuration;
use tracing::{info, debug, error};

#[tokio::main]
async fn main(){
    info!("Start");
    let configuration = read_configuration().await;
    debug!("Configuration: {:?}", &configuration);

    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let duration = time::Duration::from_secs(
        configuration.get_pull_time().into()); // 5 minutes
    let imap_server = configuration.get_imap_server();
    let matrix_client = configuration.get_matrix_client();

    loop{
        match imap_server.get_unread_mails().await{
            Ok(mails) => {
                for mail in mails{
                    debug!("{}", &mail);
                    match matrix_client.post(&mail.to_string()).await{
                        Ok(response) => debug!(response),
                        Err(error_message) => error!(error_message)

                    }
                }
            },
            Err(message) => error!("{}", message),
        }
        tokio::time::sleep(duration).await;
    }

}

async fn read_configuration() -> Configuration{
    let content = match tokio::fs::read_to_string("config.yml")
        .await {
            Ok(value) => value,
            Err(e) => {
                println!("Error with config file `config.yml`: {}",
                    e.to_string());
                process::exit(0);
            }
        };
    match Configuration::new(&content){
        Ok(configuration) => configuration,
        Err(e) => {
            println!("Error with config file `config.yml`: {}",
                e.to_string());
            process::exit(0);
        }
    }
}
