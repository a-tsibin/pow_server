use std::sync::Arc;

use anyhow::Result;
use common::{
    challenge::Challenge,
    connection::Connection,
    proto::*,
};
use log::info;
use tokio::net::TcpListener;

use crate::{
    cache::challenge_cache::ChallengeCache,
    ServerConfig,
    WowService,
};

pub struct Server<C: ChallengeCache> {
    cfg:         ServerConfig,
    wow_service: Arc<WowService>,
    cache:       C,
}

impl<C: ChallengeCache + 'static> Server<C> {
    pub fn new(cfg: ServerConfig, wow_service: WowService, cache: C) -> Server<C> {
        Server {
            cfg,
            wow_service: Arc::new(wow_service),
            cache,
        }
    }

    pub async fn start(&self) -> Result<()> {
        let addr = format!("127.0.0.1:{}", self.cfg.port);
        info!("Starting server on {}", addr);
        let listener = TcpListener::bind(addr).await?;
        info!("Server started");

        while let Ok((stream, socket)) = listener.accept().await {
            info!("New incoming connection: {}", socket);
            let service = Arc::clone(&self.wow_service);
            let difficulty = self.cfg.challenge_difficulty;
            let cache = self.cache.clone();
            tokio::spawn(async move {
                let mut conn = Connection::new(stream);
                Server::<C>::handle(&mut conn, service, difficulty, cache).await
            });
        }
        Ok(())
    }

    async fn handle(
        connection: &mut Connection,
        wow_service: Arc<WowService>,
        challenge_difficulty: u8,
        cache: C,
    ) -> Result<()> {
        info!("New client connected");
        let welcome_msg: InitMessage = connection.receive().await?;

        if let Some(solution) = welcome_msg.solution {
            Self::handle_solution_present(
                connection,
                &wow_service,
                &cache,
                &welcome_msg,
                challenge_difficulty,
                &solution,
            )
            .await?;
        } else {
            Self::handle_solution_absent(connection, challenge_difficulty, &cache, &welcome_msg)
                .await?;
        }

        Ok(())
    }

    async fn handle_solution_present(
        connection: &mut Connection,
        wow_service: &Arc<WowService>,
        cache: &C,
        welcome_msg: &InitMessage,
        challenge_difficulty: u8,
        solution: &[u8; SIZE],
    ) -> Result<()> {
        info!(
            "Solution found in the init message for client: {}",
            welcome_msg.client_id
        );

        if let Some(cached) = cache.get(&welcome_msg.client_id) {
            info!(
                "Solution found in cache for client: {}",
                welcome_msg.client_id
            );
            cached.check_solution(solution)?;
            Self::send_quote(connection, wow_service).await?;
        } else {
            info!(
                "Solution not found in cache for client: {}. Preparing new...",
                welcome_msg.client_id
            );
            Self::send_challenge(connection, challenge_difficulty, cache, welcome_msg).await?;
        }

        connection.close().await?;
        Ok(())
    }

    async fn handle_solution_absent(
        connection: &mut Connection,
        challenge_difficulty: u8,
        cache: &C,
        welcome_msg: &InitMessage,
    ) -> Result<()> {
        info!("Solution not found in the init message. Preparing new challenge...");
        Self::send_challenge(connection, challenge_difficulty, cache, welcome_msg).await?;
        connection.close().await?;
        Ok(())
    }

    async fn send_quote(connection: &mut Connection, wow_service: &Arc<WowService>) -> Result<()> {
        let quote = wow_service.get_random_quote()?;
        let quote_msg = QuoteMessage { quote };
        connection.send(&quote_msg).await?;
        Ok(())
    }

    async fn send_challenge(
        connection: &mut Connection,
        challenge_difficulty: u8,
        cache: &C,
        welcome_msg: &InitMessage,
    ) -> Result<()> {
        let challenge = Challenge::new(challenge_difficulty);
        cache.set(welcome_msg.client_id, challenge.clone());
        let challenge_msg = ChallengeMessage::from(&challenge);
        connection.send(&challenge_msg).await?;
        Ok(())
    }
}
