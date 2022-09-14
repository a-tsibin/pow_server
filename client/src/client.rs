use crate::ClientConfig;
use anyhow::Result;
use common::challenge::Challenge;
use common::connection::Connection;
use common::proto::*;
use log::info;
use tokio::net::TcpStream;

pub struct Client {
    cfg: ClientConfig,
}

impl Client {
    pub fn new(cfg: ClientConfig) -> Client {
        Client { cfg }
    }

    pub async fn start(self) -> Result<()> {
        info!("Connecting to {}", self.cfg.get_address());
        let mut tcp_stream = TcpStream::connect(self.cfg.get_address()).await?;
        let mut connection = Connection::new(&mut tcp_stream);
        info!("Waiting for challenge...");
        let challenge_msg: ChallengeMessage = connection.receive().await?;
        let challenge = Challenge::from(challenge_msg);
        info!("Challenge received. Solving...");
        let solution = challenge.solve();
        let solution_msg = SolutionMessage { solution };
        info!("Challenge solved. Sending solution");
        connection.send(&solution_msg).await?;
        let quote: QuoteMessage = connection.receive().await?;
        info!("Received quote: {}", quote.quote);
        Ok(())
    }
}
