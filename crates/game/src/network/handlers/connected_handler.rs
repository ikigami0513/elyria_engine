use cgmath::Vector3;
use common::player::PlayerInfo;
use engine::{core::frame_context::FrameContext, graphics::{animation::AnimationComponent, sprite::SpriteCreator}, world::components::TransformComponent};
use uuid::Uuid;

use crate::{gamestate::GameStateComponent, network::{event::NetworkEvent, handlers::handler::Handler}, player::{Direction, DistantPlayerComponent, State}};

pub struct ConnectedHandler;

impl Handler for ConnectedHandler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent) {
        if let Some(id_str) = event.data.get("player_id") {
            match Uuid::parse_str(id_str) {
                Ok(uuid) => {
                    println!("✅ Received and parsed player UUID: {}", uuid);

                    if let Some(gamestates_map) = ctx.world.get_components_mut::<GameStateComponent>() {
                        if let Some(gamestate) = gamestates_map.values_mut().next() {
                            gamestate.player_id = Some(uuid);
                            println!("Player UUID {} stored in GameStateComponent.", uuid);
                        } else {
                            eprintln!("❌ No GameStateComponent found in the world to update.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse UUID from string '{}': {}", id_str, e);
                }
            }
        } else {
            eprintln!("❌ 'player_id' key not found in the 'connected' event data.");
        }

        if let Some(players_json) = event.data.get("existing_players") {
            match serde_json::from_str::<Vec<PlayerInfo>>(players_json) {
                Ok(players) => {
                    for player_data in players {
                        if let Ok(player_id) = Uuid::parse_str(&player_data.id) {
                            let distant_player_entity = ctx.world.new_entity();

                            let mut transform_comp = TransformComponent::new();
                            transform_comp.transform.set_local_position(Vector3::new(player_data.x, player_data.y, player_data.z));

                            let distant_player_component = DistantPlayerComponent {
                                player_id,
                                speed: 100.0,
                                direction: Direction::DOWN,
                                state: State::IDLE,
                                target_position: None
                            };

                            let mut anim_comp = AnimationComponent::new();
                            anim_comp.play("player_base_idle_down");

                            ctx.world.add_component(distant_player_entity, transform_comp);
                            ctx.world.add_component(distant_player_entity, distant_player_component);
                            ctx.world.add_component(distant_player_entity, SpriteCreator::from_sprite(ctx.spritesheet_manager.get("player_base").unwrap(), "idle_down_0").unwrap());
                            ctx.world.add_component(distant_player_entity, anim_comp);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Échec de la désérialisation du JSON des joueurs existants : {}", e);
                }
            }
        }
    }
}