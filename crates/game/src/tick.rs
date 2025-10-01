use common::message::Message;
use engine::{
    core::frame_context::FrameContext, 
    world::{
        components::TransformComponent, entity::Entity, system::System
    }
};
use tokio::sync::mpsc;
use crate::{gamestate::GameStateComponent, player::{Direction, PlayerComponent, State}};

const TICK_INVERTAL: f32 = 1.0 / 20.0;

pub struct TickSystem {
    network_tx: mpsc::Sender<Message>,
    tick_timer: f32
}

impl TickSystem {
    pub fn new(network_tx: mpsc::Sender<Message>) -> Self {
        Self {
            network_tx,
            tick_timer: 0.0
        }
    }
}

impl System for TickSystem {
    fn update(&mut self, ctx: &mut FrameContext) {
        let delta_time = ctx.time.delta_time();
        self.tick_timer += delta_time;

        if self.tick_timer >= TICK_INVERTAL {
            self.tick_timer -= TICK_INVERTAL;

            if let Some(player_comp) = ctx.world.get_components::<PlayerComponent>() {
                let player_entities: Vec<Entity> = player_comp.keys().copied().collect();

                for entity in player_entities {
                    if let (Some(player), Some(transform), Some(gamestates)) = (
                        ctx.world.get_component::<PlayerComponent>(entity),
                        ctx.world.get_component::<TransformComponent>(entity),
                        ctx.world.get_components::<GameStateComponent>()
                    ) {
                        if let Some(gamestate) = gamestates.values().next() {
                            let position = transform.transform.get_local_position();

                            let mut msg = Message::new();
                            msg.add("player_id", &gamestate.player_id.unwrap().to_string());
                            msg.add("action", "player_move");
                            msg.add("x", &position.x.to_string());
                            msg.add("y", &position.y.to_string());
                            msg.add("z", &position.z.to_string());
                            msg.add("direction", Direction::to_str(player.direction));
                            msg.add("state", State::to_str(player.state));

                            if let Err(e) = self.network_tx.try_send(msg) {
                                eprintln!("Impossible d'envoyer la mise à jour du tick au réseau: {}", e);
                            }

                            break;
                        }
                    }
                }
            }
        }
    }
}
