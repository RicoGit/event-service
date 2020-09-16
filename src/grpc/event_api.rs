//! Rust implementation or Grpc EventApi service

use log::{info, warn};
use tonic::{Request, Response, Status};

use event_api_server::EventApi;

use crate::kafka::event_queue::EventQueue;

// Contains generated Grpc entities for EventApi
tonic::include_proto!("event");

pub struct EventSvc {
    event_queue: EventQueue,
}

impl EventSvc {
    pub fn new(event_queue: EventQueue) -> Self {
        EventSvc { event_queue }
    }
}

#[tonic::async_trait]
impl EventApi for EventSvc {
    async fn handle(
        &self,
        request: Request<TrackEventRequest>,
    ) -> Result<Response<TrackEventResponse>, Status> {
        info!("Got a request: {:?}", request);

        // save to kafka
        self.event_queue
            .send(request.get_ref())
            .await
            .map_err(|err| {
                warn!("[Error] save to kafka failed, cause {:?}", err);
                Status::internal("Can't apply event, internal error")
            })?;

        let response = TrackEventResponse {
            status: track_event_response::Status::Ok.into(),
        };

        Ok(Response::new(response))
    }
}
