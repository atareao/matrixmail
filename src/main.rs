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
    let configuration = match read_configuration().await{
        Ok(configuration) => configuration,
        Err(e) =>{
            println!("Error. Can not read configuration: {}", e);
            process::exit(0);
        }
    };
    tracing_subscriber::registry()
        .with(EnvFilter::from_str(configuration.get_log_level()).unwrap())
        .with(tracing_subscriber::fmt::layer())
        .init();
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
        }
        tokio::time::sleep(duration).await;
    }

}

async fn read_configuration() -> Result<Configuration, String>{
    let content = tokio::fs::read_to_string("config.yml").await.map_err(|err|
        err.to_string())?;
    Configuration::new(&content).map_err(|err| err.to_string())
}
