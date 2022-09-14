use crate::{ServerConfig, WowService};
use anyhow::Result;
use common::challenge::Challenge;
use common::connection::Connection;
use common::proto::*;
use log::info;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Server {
    cfg: ServerConfig,
    wow_service: Arc<WowService>,
}

impl Server {
    pub fn new(cfg: ServerConfig, wow_service: WowService) -> Server {
        Server {
            cfg,
            wow_service: Arc::new(wow_service),
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = format!("127.0.0.1:{}", self.cfg.port);
        info!("Starting server on {}", addr);
        let listener = TcpListener::bind(addr).await?;
        info!("Server started");

        while let Ok((mut stream, socket)) = listener.accept().await {
            info!("New incoming connection: {}", socket);
            let service = Arc::clone(&self.wow_service);
            let difficulty = self.cfg.challenge_difficulty;
            tokio::spawn(async move {
                let mut conn = Connection::new(&mut stream);
                Server::handle(&mut conn, service, difficulty).await
            });
        }
        Ok(())
    }

    async fn handle(
        connection: &mut Connection<'_>,
        wow_service: Arc<WowService>,
        challenge_difficulty: u8,
    ) -> Result<()> {
        info!("Client connected. Preparing challenge...");
        let challenge = Challenge::new(challenge_difficulty);
        let challenge_msg = ChallengeMessage::from(&challenge);
        connection.send(&challenge_msg).await?;
        info!("Challenge sent. Waiting for solution.");
        let solution: SolutionMessage = connection.receive().await?;
        challenge.check_solution(&solution.solution)?;
        let quote = wow_service.get_random_quote()?;
        let quote_msg = QuoteMessage { quote };
        connection.send(&quote_msg).await?;
        connection.close().await
    }
}
