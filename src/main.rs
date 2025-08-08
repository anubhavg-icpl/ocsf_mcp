use anyhow::Result;
use rmcp::{transport::stdio, ServiceExt};
use tracing_subscriber::{self, EnvFilter};

mod ocsf;
mod templates;
mod tools;

use tools::OcsfServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stderr logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting OCSF MCP Server");

    // Create an instance of our OCSF server
    let service = OcsfServer::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    tracing::info!("Server initialized, waiting for requests");
    service.waiting().await?;

    Ok(())
}
