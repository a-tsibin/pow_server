mod client;
mod config;

use anyhow::Result;
use env_logger::Target;
use log::*;
use rand::random;

use crate::{
    client::Client,
    config::ClientConfig,
};

// TODO: add request's retry logic
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Initializing client...");
    let cfg = ClientConfig::from_env()?;
    let client_id: u64 = random();
    let client = Client::new(cfg, client_id);
    client.start().await?;
    Ok(())
}
