//! Contains all structs and methods related to Event topic.

use crate::grpc::event_api::TrackEventRequest;
use anyhow::Result;
use rdkafka::producer::FutureProducer;
use std::future::Future;

struct EventQueue {
    topic: String,
    producer: FutureProducer,
}

pub async fn send_event(req: TrackEventRequest) -> Result<()> {
    todo!()
}
