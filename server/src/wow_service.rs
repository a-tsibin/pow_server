use crate::errors::ServerError;
use anyhow::{anyhow, Result};
use log::info;
use rand::seq::SliceRandom;
use std::path::Path;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

pub struct WowService {
    quotes: Vec<String>,
}

impl WowService {
    pub async fn new(quotes_path: &Path) -> Result<WowService> {
        info!("Initializing words of wisdom service...");
        let file = File::open(quotes_path).await?;
        let buf = BufReader::new(file);
        let mut lines = buf.lines();
        let mut quotes = Vec::new();
        while let Some(line) = lines.next_line().await? {
            quotes.push(line)
        }
        info!(
            "Words of wisdom service initialized, total quotes count {}",
            quotes.len()
        );
        Ok(WowService { quotes })
    }

    pub fn get_random_quote(&self) -> Result<String> {
        self.quotes
            .choose(&mut rand::thread_rng())
            .ok_or(anyhow!(ServerError::EmptyQuotesList))
            .map(|q| q.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::WowService;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_quotas_loading() {
        let mut wow_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wow_path.push("resources/wow");
        if let Ok(service) = WowService::new(wow_path.as_path()).await {
            assert!(service.get_random_quote().is_ok())
        } else {
            panic!()
        }
    }

    #[tokio::test]
    async fn test_error_if_quotes_is_empty() {
        let mut wow_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        wow_path.push("resources/wow_empty");
        if let Ok(service) = WowService::new(wow_path.as_path()).await {
            assert!(service.get_random_quote().is_err());
        } else {
            panic!()
        }
    }
}
