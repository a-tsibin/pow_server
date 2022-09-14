use anyhow::Result;
use std::env;

use std::path::PathBuf;

pub struct ServerConfig {
    pub port: i32,
    pub challenge_difficulty: u8,
    pub wow_path: PathBuf,
}

impl ServerConfig {
    pub fn new(port: i32, challenge_difficulty: u8, wow_path: String) -> ServerConfig {
        ServerConfig {
            port,
            challenge_difficulty,
            wow_path: PathBuf::from(wow_path),
        }
    }

    pub fn from_env() -> Result<ServerConfig> {
        let port = env::var("PORT")?.parse::<i32>()?;
        let challenge_difficulty = env::var("DIFFICULTY")?.parse::<u8>()?;
        let wow_path = env::var("PATH")?;
        Ok(ServerConfig::new(port, challenge_difficulty, wow_path))
    }
}
