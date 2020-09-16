//! Provides high-level kafka Api.
//!
//! todo example

use std::collections::HashMap;

pub mod event_queue;
pub mod kafka_client;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KafkaConfig {
    pub producer: ProducerConfig,
    pub event_topic: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ProducerConfig {
    pub bootstrap_servers: String,
    pub options: HashMap<String, String>,
}
