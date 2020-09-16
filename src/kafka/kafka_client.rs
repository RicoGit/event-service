use crate::kafka::KafkaConfig;
use rdkafka::producer::FutureProducer;
use std::io;

use anyhow::{Context, Result};
use rdkafka::error::KafkaError;
use rdkafka::ClientConfig;

pub struct KafkaClient {
    config: KafkaConfig,
}

impl KafkaClient {
    /// Creates new instance for current config
    pub fn new(config: KafkaConfig) -> Self {
        KafkaClient { config }
    }

    /// Starts kafka client processing.
    pub async fn start(self) -> Result<()> {
        let producer = self.create_producer()?;
        self.start_process(producer)
            .await
            .context("Can't start KafkaClient")
    }

    /// This methods block a current thread.
    pub async fn start_process(self, producer: FutureProducer) -> Result<(), KafkaError> {
        // todo add channel or stream to receive messages
        unimplemented!()
    }

    /// Creates producer.
    pub fn create_producer(&self) -> Result<FutureProducer, KafkaError> {
        let conf = &self.config.producer;
        let mut builder = ClientConfig::new();
        builder.set("bootstrap.servers", &conf.bootstrap_servers);

        for (k, v) in &conf.options {
            builder.set(k, v);
        }

        builder.create()
    }
}
