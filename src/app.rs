use anyhow::Error;
use log::info;

use crate::grpc;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AppConfig {
    // pub kafka: kafka::KafkaConfig,
    pub grpc: grpc::GrpcConfig,
}

/// Starts server for processing Grpc/Rest events. Connects and writes events to Kafka.
/// Blocks the current thread.
pub async fn start(config_path: &str) -> Result<(), Error> {
    let config = read_config(config_path);
    info!("AppConfig: {:?}", config);
    run(config).await
}

pub async fn run(config: AppConfig) -> Result<(), Error> {
    // let kafka = KafkaClient::new(config.kafka);
    // kafka.start().context("Can't start KafkaClient")

    grpc::start(config.grpc).await
}

/// Reads config from the specified path, allows override some properties vie env variables.
pub fn read_config(path: &str) -> AppConfig {
    use config::{Environment, File};

    let mut settings = config::Config::default();
    settings
        .merge(File::with_name(path.into()))
        .expect("Error while loading config file");
    settings.merge(Environment::with_prefix("app")).unwrap();
    settings
        .try_into::<AppConfig>()
        .expect("Config is invalid.")
}
