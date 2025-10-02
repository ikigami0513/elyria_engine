#[link(name = "shell32")]
unsafe extern "C" {}

mod handler;
mod player;

use std::collections::HashMap;
use std::sync::Arc;
use common::player::PlayerInfo;
use engine::world::components::TransformComponent;
use engine::world::world::World;
use tokio::net::tcp::OwnedWriteHalf;
use tokio::sync::Mutex;

use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use common::message::Message;
use uuid::Uuid;

use crate::handler::{Handler, HandlerContext};
use crate::player::{PlayerComponent, PlayerMoveHandler};

struct ElyriaServer {
    handlers: Mutex<HashMap<String, Box<dyn Handler + Send + Sync>>>,
    world: Mutex<World>,
    clients: Mutex<HashMap<Uuid, Arc<Mutex<OwnedWriteHalf>>>>
}

impl ElyriaServer {
    fn new() -> Self {
        Self {
            handlers: Mutex::new(HashMap::new()),
            world: Mutex::new(World::new()),
            clients: Mutex::new(HashMap::new())
        }
    }

    async fn handle_client(&self, socket: TcpStream) {
        println!("Client connecté depuis : {}", socket.peer_addr().unwrap());

        let player_id = Uuid::new_v4();

        let (mut reader, writer) = socket.into_split();
        let writer = Arc::new(Mutex::new(writer));

        {
            self.clients.lock().await.insert(player_id, writer.clone());
        }

        let initial_transform;
        
        {
            let mut world = self.world.lock().await;
            let player_entity = world.new_entity();

            let transform = TransformComponent::new();
            initial_transform = transform.clone();

            world.add_component(player_entity, transform);
            world.add_component(player_entity, PlayerComponent::new(player_id));

            println!("Joueur créé avec l'ID: {}, Entité: {:?}", player_id, player_entity);
            println!("Composants joueurs actuels: {:?}", world.get_components::<PlayerComponent>());
        }

        {
            let mut existing_players_info = Vec::new();

            {
                let world = self.world.lock().await;

                if let Some(player_components) = world.get_components::<PlayerComponent>() {
                    for (entity, player_comp) in player_components.iter() {
                        if player_comp.id == player_id {
                            continue;
                        }

                        if let Some(transform_comp) = world.get_component::<TransformComponent>(*entity) {
                            existing_players_info.push(PlayerInfo {
                                id: player_comp.id.to_string(),
                                x: transform_comp.transform.get_local_position().x,
                                y: transform_comp.transform.get_local_position().y,
                                z: transform_comp.transform.get_local_position().z
                            });
                        }
                    }
                }
            }

            let existing_players_json = serde_json::to_string(&existing_players_info).unwrap();

            let mut welcome_message = Message::new();
            welcome_message.add("action", "connected");
            welcome_message.add("player_id", &player_id.to_string());
            welcome_message.add("existing_players", &existing_players_json);

            if let Ok(bytes) = welcome_message.to_bytes() {
                let mut w = writer.lock().await;
                if w.write_u32(bytes.len() as u32).await.is_ok() {
                    let _ = w.write_all(&bytes).await;
                }
            }
        }

        {
            let mut broadcast_message = Message::new();
            broadcast_message.add("action", "new_distant_player");
            broadcast_message.add("player_id", &player_id.to_string());
            broadcast_message.add("x", &initial_transform.transform.get_local_position().x.to_string());
            broadcast_message.add("Y", &initial_transform.transform.get_local_position().y.to_string());
            broadcast_message.add("z", &initial_transform.transform.get_local_position().z.to_string());

            if let Ok(bytes_to_broadcast) = broadcast_message.to_bytes() {
                let clients_map = self.clients.lock().await;

                for (id, client_writer) in clients_map.iter() {
                    if *id != player_id {
                        let mut w = client_writer.lock().await;
                        if w.write_u32(bytes_to_broadcast.len() as u32).await.is_ok() {
                            let _ = w.write_all(&bytes_to_broadcast).await;
                        }
                    }
                }
            }
        }

        loop {
            let msg_len = match reader.read_u32().await {
                Ok(0) | Err(_) => {
                    println!("Client déconnecté.");
                    break;
                }
                Ok(len) => len
            };

            let mut msg_buffer = vec![0; msg_len as usize];
            if let Err(e) = reader.read_exact(&mut msg_buffer).await {
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
                                clients: &self.clients,
                                current_player_id: player_id
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
