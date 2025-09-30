use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use common::message::Message;

pub struct Client {
    stream: TcpStream
}

#[allow(dead_code)]
impl Client {
    pub async fn connect(addr: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let stream = TcpStream::connect(addr).await?;
        println!("✅ Connected to server at {}", addr);
        Ok(Self { stream })
    }

    pub async fn send(&mut self, message: &Message) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let bytes = message.to_bytes()?;

        let len = bytes.len() as u32;
        self.stream.write_u32(len).await?;

        self.stream.write_all(&bytes).await?;

        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
        let msg_len = match self.stream.read_u32().await {
            Ok(len) => len,
            Err(e) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Err("Le serveur a fermé la connexion".into());
            }
            Err(e) => return Err(e.into())
        };

        if msg_len == 0 {
            return Err("Le serveur a fermé la connexion (longueur de message 0).".into());
        }

        let mut buffer = vec![0; msg_len as usize];
        self.stream.read_exact(&mut buffer).await?;
        let message = Message::from_bytes(&buffer)?;

        Ok(message)
    }
}
