use anyhow::Result;
use common::{
    challenge::Challenge,
    connection::Connection,
    proto::*,
};
use log::info;
use tokio::net::TcpStream;

use crate::ClientConfig;

pub struct Client {
    cfg: ClientConfig,
    id:  ClientId,
}

impl Client {
    pub fn new(cfg: ClientConfig, client_id: ClientId) -> Client {
        Client { cfg, id: client_id }
    }

    pub async fn start(self) -> Result<()> {
        info!("Connecting to {}", self.cfg.get_address());
        let mut connection = self.establish_connection().await?;

        self.send_initial_message(&mut connection).await?;

        let challenge = self.receive_challenge(&mut connection).await?;
        connection.close().await?;

        info!("Solving challenge...");
        let solution = challenge.solve();
        connection = self.establish_connection().await?;
        self.send_solution(&mut connection, solution).await?;
        let quote: QuoteMessage = connection.receive().await?;

        info!("Received quote: {}", quote.quote);

        Ok(())
    }

    async fn establish_connection(&self) -> Result<Connection> {
        let tcp_stream = TcpStream::connect(self.cfg.get_address()).await?;
        Ok(Connection::new(tcp_stream))
    }

    async fn send_initial_message(&self, connection: &mut Connection) -> Result<()> {
        info!("Sending initial message");
        let init_msg = InitMessage::new(self.id, None);
        connection.send(&init_msg).await
    }

    async fn receive_challenge(&self, connection: &mut Connection) -> Result<Challenge> {
        info!("Waiting for challenge...");
        let challenge_msg: ChallengeMessage = connection.receive().await?;
        info!("Challenge received. Solving...");
        Ok(Challenge::from(challenge_msg))
    }

    async fn send_solution(&self, connection: &mut Connection, solution: [u8; SIZE]) -> Result<()> {
        let solution_msg = InitMessage::new(self.id, Some(solution));
        info!("Challenge solved. Sending solution");
        connection.send(&solution_msg).await
    }
}
