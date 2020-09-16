//! Rust implementation or Grpc EventApi service

use event_api_server::EventApi;
use log::info;
use tonic::{Request, Response, Status};

// Contains generated Grpc entities for EventApi
tonic::include_proto!("event");

#[derive(Debug, Default)]
pub struct EventSvc {}

#[tonic::async_trait]
impl EventApi for EventSvc {
    async fn handle(
        &self,
        request: Request<TrackEventRequest>,
    ) -> Result<Response<TrackEventResponse>, Status> {
        // todo implement !
        info!("Got a request: {:?}", request);

        let response = TrackEventResponse {
            status: track_event_response::Status::Ok.into(),
        };

        Ok(Response::new(response))
    }
}
