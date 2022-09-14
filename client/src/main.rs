mod client;
mod config;

use crate::client::Client;
use crate::config::ClientConfig;
use anyhow::Result;
use env_logger::Target;
use log::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Initializing client...");
    let cfg = ClientConfig::from_env()?;
    let client = Client::new(cfg);
    client.start().await?;
    Ok(())
}
