use async_trait::async_trait;
use cgmath::vec3;
use common::{message::Message, player::{Direction, State}};
use engine::world::{components::{Component, TransformComponent}, entity::Entity};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

use crate::handler::{MessageHandler, MessageHandlerContext};

#[derive(Debug)]
pub struct PlayerComponent {
    pub id: Uuid,
    pub direction: Direction,
    pub state: State
}

impl PlayerComponent {
    pub fn new(id: Uuid) -> Self {
        Self {
            id,
            direction: Direction::DOWN,
            state: State::IDLE
        }
    }
}

impl Component for PlayerComponent {}

pub struct PlayerMoveHandler;

#[async_trait]
impl MessageHandler for PlayerMoveHandler {
    async fn handle<'ctx>(&self, ctx: MessageHandlerContext<'ctx>) {
        let data = ctx.message.get_data();

        let target_player_id = match data.get("player_id").and_then(|s| s.parse::<Uuid>().ok()) {
            Some(id) => id,
            None => {
                eprintln!("ID de joueur manquant ou invalide dans le message.");
                return;
            }
        };

        let player_entity: Option<Entity> = {
            if let Some(player_comps) = ctx.world.get_components::<PlayerComponent>() {
                player_comps
                    .iter()
                    .find(|(_entity, component)| component.id == target_player_id)
                    .map(|(entity, _component)| *entity)
            }
            else {
                None
            }
        };
        
        if let Some(entity) = player_entity {
            let x_pos = data.get("x").and_then(|val| val.parse::<f32>().ok());
            let y_pos = data.get("y").and_then(|val| val.parse::<f32>().ok());
            let z_pos = data.get("z").and_then(|val| val.parse::<f32>().ok());
            let state = data.get("state").and_then(|val| val.parse::<State>().ok()).unwrap();
            let direction = data.get("direction").and_then(|val| val.parse::<Direction>().ok()).unwrap();

            if let (Some(x), Some(y), Some(z)) = (x_pos, y_pos, z_pos) {
                if let Some((player_comp, transform_comp)) = ctx.world.get_components_mut_pair::<PlayerComponent, TransformComponent>(entity) {
                    transform_comp.transform.set_local_position(vec3(x, y, z));
                    player_comp.state = state;
                    player_comp.direction = direction;
                }
                else {
                    return;
                }

                let mut broadcast_message = Message::new();
                broadcast_message.add("action", "player_moved");
                broadcast_message.add("player_id", &target_player_id.to_string());
                broadcast_message.add("x", &x.to_string());
                broadcast_message.add("y", &y.to_string());
                broadcast_message.add("z", &z.to_string());
                broadcast_message.add("direction", Direction::to_str(direction));
                broadcast_message.add("state", State::to_str(state));

                let bytes = match broadcast_message.to_bytes() {
                    Ok(b) => b,
                    Err(_) => return
                };

                let clients_map = ctx.clients.lock().await;
                for (id, client_writer) in clients_map.iter() {
                    if *id != ctx.current_player_id {
                        let owned_id = *id;
                        let writer_arc = client_writer.clone();
                        let bytes_clone = bytes.clone();

                        tokio::spawn(async move {
                            let mut writer = writer_arc.lock().await;
                            if writer.write_u32(bytes_clone.len() as u32).await.is_ok() {
                                if let Err(e) = writer.write_all(&bytes_clone).await {
                                    eprintln!("Erreur de broadcast vers le client {}: {}", owned_id, e);
                                }
                            }
                        });
                    }
                }
            }
        }
    }
}
