#[macro_use]
extern crate serde_derive;

use std::env;

use log::{error, info};

pub mod app;
pub mod grpc;
pub mod kafka;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    info!(
        "Event Service version is {} is staring ...",
        env!("CARGO_PKG_VERSION")
    );

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("[Error] Config path is required. Put the full path to the config file as the first parameter.");
        ::std::process::exit(1);
    }

    info!("Preparing to start the processing");
    app::start(&args[1]).await?;
    info!("Processing was shutdown");

    Ok(())
}
