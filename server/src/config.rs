use std::{
    env,
    path::PathBuf,
};

use anyhow::Result;

pub struct ServerConfig {
    pub port:                 i32,
    pub challenge_difficulty: u8,
    pub wow_path:             PathBuf,
    pub cache_capacity:       u64,
}

impl ServerConfig {
    pub fn new(
        port: i32,
        challenge_difficulty: u8,
        wow_path: String,
        cache_capacity: u64,
    ) -> ServerConfig {
        ServerConfig {
            port,
            challenge_difficulty,
            wow_path: PathBuf::from(wow_path),
            cache_capacity,
        }
    }

    pub fn from_env() -> Result<ServerConfig> {
        let port = env::var("PORT")?.parse::<i32>()?;
        let challenge_difficulty = env::var("DIFFICULTY")?.parse::<u8>()?;
        let wow_path = env::var("QUOTES_PATH")?;
        let cache_capacity = env::var("CACHE_CAPACITY")?.parse::<u64>()?;
        Ok(ServerConfig::new(
            port,
            challenge_difficulty,
            wow_path,
            cache_capacity,
        ))
    }
}
