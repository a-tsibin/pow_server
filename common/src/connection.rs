use anyhow::Result;
use log::info;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Connection<'a> {
    tcp_stream: &'a mut TcpStream,
}

impl<'a> Connection<'a> {
    pub fn new(stream: &'a mut TcpStream) -> Connection {
        Connection { tcp_stream: stream }
    }

    pub async fn receive<T: DeserializeOwned>(&mut self) -> Result<T> {
        let mut buf = vec![0; 1024];
        let _bytes = self.tcp_stream.read(&mut buf).await?;
        let msg: T = bincode::deserialize(&buf)?;
        Ok(msg)
    }

    pub async fn send<T: Serialize>(&mut self, msg: &T) -> Result<()> {
        let data = bincode::serialize(msg)?;
        self.tcp_stream.write_all(&data).await?;
        Ok(())
    }

    pub async fn close(&mut self) -> Result<()> {
        info!("Closing connection");
        Ok(self.tcp_stream.shutdown().await?)
    }
}
