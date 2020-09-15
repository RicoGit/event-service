use tonic::{transport::Server, Request, Response, Status};

use events_grpc::event_api_server::{EventApi, EventApiServer};
use events_grpc::{TrackEventRequest, TrackEventResponse};
use log::info;

/// Contains generated Grpc entities for EventApi
pub mod events_grpc {
    // The string specified here must match the proto package name
    tonic::include_proto!("event");
}

#[derive(Debug, Default)]
pub struct EventSvc {}

#[tonic::async_trait]
impl EventApi for EventSvc {
    async fn handle(
        &self,
        request: Request<TrackEventRequest>,
    ) -> Result<Response<TrackEventResponse>, Status> {
        info!("Got a request: {:?}", request);

        let response = events_grpc::TrackEventResponse {
            status: events_grpc::track_event_response::Status::Ok.into(),
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Event service is starting...");

    let addr = "[::1]:50051".parse()?;
    let event_svc = EventSvc::default();

    Server::builder()
        .add_service(EventApiServer::new(event_svc))
        .serve(addr)
        .await?;

    Ok(())
}
