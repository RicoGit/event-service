use anyhow::Result;
use log::info;
use tokio::prelude;

use crate::kafka::kafka_client::KafkaClient;
use crate::{grpc, kafka};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct AppConfig {
    pub kafka: kafka::KafkaConfig,
    pub grpc: grpc::GrpcConfig,
}

/// Starts server for processing Grpc/Rest events. Connects and writes events to Kafka.
/// Blocks the current thread.
pub async fn start(config_path: &str) -> Result<()> {
    let config = read_config(config_path);
    info!("AppConfig: {:?}", config);
    run(config).await
}

pub async fn run(config: AppConfig) -> Result<()> {
    // starting all services as long-running futures, if any finishes other will be stopped
    tokio::select! {
        grpc = grpc::start(config.grpc) => grpc?,
        kafka = KafkaClient::new(config.kafka).start() => kafka?
    };
    info!("All services are initializing");
    Ok(())
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
