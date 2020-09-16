//! Provides high-level kafka Api.
//!
//! ## Example
//!
//! ```no_run
//!     use kafka::event_queue::EventQueue;
//!
//!     let eventQueue = EventQueue::new(config);
//!     eventQueue.send(event).await
//! ```

use std::collections::HashMap;

pub mod event_queue;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct KafkaConfig {
    pub producer: ProducerConfig,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ProducerConfig {
    pub bootstrap_servers: String,
    pub options: HashMap<String, String>,
    event_topic: String,
}
