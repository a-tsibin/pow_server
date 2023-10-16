mod cache;
mod config;
mod errors;
mod server;
mod wow_service;

use anyhow::Result;
use env_logger::Target;
use log::*;

use crate::{
    config::ServerConfig,
    server::Server,
    wow_service::WowService,
};

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
    let cache = cache::in_memory_cache::InMemoryCache::new(cfg.cache_capacity);
    let server = Server::new(cfg, wow_service, cache);
    server.start().await?;
    Ok(())
}
