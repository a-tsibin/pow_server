use std::env;

use anyhow::Result;

pub struct ClientConfig {
    pub host: String,
    pub port: i32,
}

impl ClientConfig {
    pub fn new(host: String, port: i32) -> ClientConfig {
        ClientConfig { host, port }
    }

    pub fn from_env() -> Result<ClientConfig> {
        let host = env::var("HOST")?;
        let port = env::var("PORT")?.parse::<i32>()?;
        Ok(ClientConfig::new(host, port))
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
