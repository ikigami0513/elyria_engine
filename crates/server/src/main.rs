#[link(name = "shell32")]
unsafe extern "C" {}

mod handler;
mod player;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use engine::world::components::TransformComponent;
use engine::world::world::World;
use tokio::sync::Mutex;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::AsyncReadExt;
use common::message::Message;

use crate::handler::{Handler, HandlerContext};
use crate::player::{PlayerComponent, PlayerMoveHandler};

struct ElyriaServer {
    handlers: Mutex<HashMap<String, Box<dyn Handler + Send + Sync>>>,
    world: Mutex<World>,
    next_player_id: AtomicU32
}

impl ElyriaServer {
    fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
            world: Mutex::new(World::new()),
            next_player_id: AtomicU32::new(1)
        }
    }

    async fn handle_client(&self, mut socket: TcpStream) {
        println!("Client connecté depuis : {}", socket.peer_addr().unwrap());

        let player_id = self.next_player_id.fetch_add(1, Ordering::SeqCst);
        
        {
            let mut world = self.world.lock().await;
            let player_entity = world.new_entity();
            world.add_component(player_entity, TransformComponent::new());
            world.add_component(player_entity, PlayerComponent::new(player_id));

            println!("Joueur créé avec l'ID: {}, Entité: {:?}", player_id, player_entity);
            println!("Composants joueurs actuels: {:?}", world.get_components::<PlayerComponent>());
        }

        loop {
            let msg_len = match socket.read_u32().await {
                Ok(0) | Err(_) => {
                    println!("Client déconnecté.");
                    break;
                }
                Ok(len) => len
            };

            let mut msg_buffer = vec![0; msg_len as usize];
            if let Err(e) = socket.read_exact(&mut msg_buffer).await {
                eprintln!("Erreur en lisant le message : {}", e);
                break;
            }

            match Message::from_bytes(&msg_buffer) {
                Ok(message) => {
                    let data = message.get_data();
                    if let Some(action) = data.get("action") {
                        let handlers_map = self.handlers.lock().await;
                        if let Some(handler) = handlers_map.get(action) {
                            let mut world_guard = self.world.lock().await;
                            let ctx = HandlerContext {
                                message: &message,
                                world: &mut world_guard,
                                socket: &mut socket
                            };

                            handler.handle(ctx).await;
                        }
                    }
                 }
                Err(e) => {
                    eprintln!("Erreur de désérialisation : {}", e);
                }
            }
        }
    }

    async fn run(self: Arc<Self>, addr: &str) {
        let listener = TcpListener::bind(addr).await.unwrap();
        println!("Server listening to {}", addr);

        loop {
            match listener.accept().await {
                Ok((socket, _addr)) => {
                    let server_clone = Arc::clone(&self);
                    tokio::spawn(async move {
                        server_clone.handle_client(socket).await;
                    });
                }
                Err(e) => eprintln!("Error accept : {:?}", e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server = Arc::new(ElyriaServer::new());
    server.world.lock().await.register_component::<PlayerComponent>();
    server.handlers.lock().await.insert("player_move".to_string(), Box::new(PlayerMoveHandler));
    server.run("127.0.0.1:8080").await;
}
