//! Grpc server
//!
//! ## Example
//!
//! ```no_run
//!     grpc::start("/full/path/to/config.yml").await
//! ```

use anyhow::{Context, Result};
use tonic::transport::Server;

use event_api::event_api_server::EventApiServer;

use crate::grpc::event_api::EventSvc;
use crate::kafka::event_queue::EventQueue;

pub mod event_api;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct GrpcConfig {
    address: String,
}

/// Starts Grpc server
pub async fn start(grpc_config: GrpcConfig, event_queue: EventQueue) -> Result<()> {
    let address = grpc_config.address.parse()?;
    let event_svc = EventSvc::new(event_queue);

    Server::builder()
        .add_service(EventApiServer::new(event_svc))
        .serve(address)
        .await
        .context("Can't start GrpcServer")
}
