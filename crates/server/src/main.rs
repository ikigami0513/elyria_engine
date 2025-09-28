#[link(name = "shell32")]
unsafe extern "C" {}

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

struct ElyriaServer;

impl ElyriaServer {
    fn new() -> Self {
        Self
    }

    async fn handle_client(&self, mut socket: TcpStream) {
        println!("Client connected!");

        let mut buffer = [0u8; 1024];
        loop {
            match socket.read(&mut buffer).await {
                Ok(0) => {
                    println!("Client disconnected");
                    break;
                }
                Ok(n) => {
                    println!("Received {} bytes: {:?}", n, &buffer[..n]);
                    if let Err(e) = socket.write_all(&buffer[..n]).await {
                        eprintln!("Erreur en renvoyant les données : {:?}", e);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error reading socket : {:?}", e);
                    break;
                }
            }
        }
    }

    async fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server listening to {}", addr);

        loop {
            match listener.accept().await {
                Ok((socket, _addr)) => {
                    let server_clone = self.clone();
                    tokio::spawn(async move {
                        server_clone.handle_client(socket).await;
                    });
                }
                Err(e) => eprintln!("Error accept : {:?}", e)
            }
        }
    }
}

impl Clone for ElyriaServer {
    fn clone(&self) -> Self {
        Self
    }
}

#[tokio::main]
async fn main() {
    let server = ElyriaServer::new();
    server.run("127.0.0.1:8080").await;
}
