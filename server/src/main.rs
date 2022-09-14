mod config;
mod errors;
mod server;
mod wow_service;

use crate::config::ServerConfig;
use crate::server::Server;
use crate::wow_service::WowService;
use anyhow::Result;
use env_logger::Target;
use log::*;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::builder()
        .filter_level(LevelFilter::Info)
        .target(Target::Stdout)
        .init();
    info!("Server initializing...");
    let cfg = ServerConfig::from_env()?;
    info!("Config successfully loaded");
    let wow_service = WowService::new(cfg.wow_path.as_path()).await?;
    info!("Words of wisdom service initialized");
    let server = Server::new(cfg, wow_service);
    server.start().await?;
    Ok(())
}
