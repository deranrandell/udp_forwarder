use std::env;
use log::{error, info};
use tokio::signal;
use udp_forwarder::{UdpForwarder, Config};
use udp_forwarder::error::AppError;
use udp_forwarder::config::load_config;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Initialize logging
    env_logger::init();

    // Load configuration (source and destination addresses)
    let config = load_config()?;

    // Create the UDP forwarder
    let forwarder = UdpForwarder::new(config.source_addr, config.destination_addr);

    // Run the UDP forwarding in a background task
    let forwarder_task = tokio::spawn(async move {
        if let Err(e) = forwarder.run().await {
            error!("Error in UDP forwarding: {}", e);
        }
    });

    // Wait for Ctrl+C signal to terminate the application
    tokio::select! {
        _ = forwarder_task => {},
        _ = signal::ctrl_c() => {
            info!("Ctrl+C received, shutting down.");
        },
    }

    Ok(())
}
