//! Contains all structs and methods related to Event topic.

use std::future::Future;

use anyhow::{Context, Result};
use prost::bytes::BufMut;
use prost::Message;
use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use tokio::time::Duration;

use crate::grpc::event_api::TrackEventRequest;
use crate::kafka::ProducerConfig;

struct EventQueue {
    config: ProducerConfig,
    producer: FutureProducer,
}

impl EventQueue {
    /// Creates new Event Producer
    fn new(config: ProducerConfig) -> Result<Self> {
        EventQueue::create_producer(&config)
            .map(|producer| EventQueue { config, producer })
            .context("Can't create Kafka producer for events")
    }

    /// Pushes event to Kafka topic, returns partition and offset of the message in success case.
    pub async fn send(&mut self, req: TrackEventRequest) -> Result<(i32, i64)> {
        let payload_bytes = to_bytes(req)?;
        let kafka_message = FutureRecord::to(&self.config.event_topic)
            .key(&()) // empty key it's ok
            .payload(&payload_bytes);

        // if queue is full waits for 10 sec, then interrupts and returns [RDKafkaError::QueueFull]
        self.producer
            .send(kafka_message, Duration::from_secs(10))
            .await
            .map_err(|(err, _msg)| err.into())
    }

    /// Build kafka producer for Events
    fn create_producer(config: &ProducerConfig) -> Result<FutureProducer, KafkaError> {
        let mut builder = ClientConfig::new();
        builder.set("bootstrap.servers", &config.bootstrap_servers);

        for (k, v) in &config.options {
            builder.set(k, v);
        }

        builder.create()
    }
}

fn to_bytes(req: TrackEventRequest) -> Result<Vec<u8>> {
    let mut buf = Vec::with_capacity(req.encoded_len());
    req.encode(&mut buf)?;
    Ok(buf.to_owned())
}
