use anyhow::Result;
use log::trace;
use serde::{
    de::DeserializeOwned,
    Serialize,
};
use tokio::{
    io::{
        AsyncReadExt,
        AsyncWriteExt,
    },
    net::TcpStream,
};

pub struct Connection {
    tcp_stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
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
        trace!("Closing connection");
        Ok(self.tcp_stream.shutdown().await?)
    }
}
