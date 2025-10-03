use cgmath::vec3;
use engine::{core::frame_context::FrameContext, graphics::{animation::AnimationComponent, sprite::SpriteCreator}, world::components::TransformComponent};
use uuid::Uuid;

use crate::{network::{event::NetworkEvent, handlers::handler::Handler}, player::DistantPlayerComponent};
use common::player::{Direction, State};

pub struct DistantPlayerMovedHandler;

impl Handler for DistantPlayerMovedHandler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent) {
        if let Some(player_id_string) = event.data.get("player_id") {
            match Uuid::parse_str(player_id_string) {
                Ok(player_id) => {
                    let mut target_entity_info = None;

                    if let Some(distant_player_comps) = ctx.world.get_components_mut::<DistantPlayerComponent>() {
                        for (entity, distant_player_comp) in distant_player_comps {
                            if distant_player_comp.player_id == player_id {
                                let x = event.data.get("x").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                                let y = event.data.get("y").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                                let z = event.data.get("z").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);

                                distant_player_comp.target_position = Some(vec3(x, y, z));

                                let new_state = event.data.get("state").and_then(|v| v.parse::<State>().ok()).unwrap();
                                let new_direction = event.data.get("direction").and_then(|v| v.parse::<Direction>().ok()).unwrap();
                                
                                if new_state != distant_player_comp.state || new_direction != distant_player_comp.direction {
                                    distant_player_comp.state = new_state;
                                    distant_player_comp.direction = new_direction;
                                    
                                    target_entity_info = Some((*entity, new_state, new_direction));
                                }

                                break;
                            }
                        }
                    }

                    if let Some((entity, new_state, new_direction)) = target_entity_info {
                        if let Some(animation_comp) = ctx.world.get_component_mut::<AnimationComponent>(entity) {
                            let anim_name = format!("player_base_{}_{}", State::to_str(new_state), Direction::to_str(new_direction));
                            animation_comp.play(&anim_name);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse UUID from string '{}': {}", player_id_string, e);
                }
            }
        }
    }
}

pub struct NewDistantPlayerHandler;

impl Handler for NewDistantPlayerHandler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent) {
        if let Some(player_id_string) = event.data.get("player_id") {
            match Uuid::parse_str(player_id_string) {
                Ok(player_id) => {
                    let distant_player_entity = ctx.world.new_entity();
            
                    let x = event.data.get("x").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                    let y = event.data.get("y").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                    let z = event.data.get("z").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);

                    let speed = event.data.get("speed").and_then(|v| v.parse::<f32>().ok()).unwrap_or(100.0);

                    let mut transform_comp = TransformComponent::new();
                    transform_comp.transform.set_local_position(vec3(x, y, z));

                    let distant_player_comp = DistantPlayerComponent {
                        player_id,
                        speed,
                        direction: Direction::DOWN,
                        state: State::IDLE,
                        target_position: None
                    };

                    let mut anim_comp = AnimationComponent::new();
                    anim_comp.play("player_base_idle_down"); 

                    ctx.world.add_component(distant_player_entity, transform_comp);
                    ctx.world.add_component(distant_player_entity, distant_player_comp);
                    ctx.world.add_component(distant_player_entity, SpriteCreator::from_sprite(ctx.spritesheet_manager.get("player_base").unwrap(), "idle_down_0").unwrap());
                    ctx.world.add_component(distant_player_entity, anim_comp);
                }
                Err(e) => {
                    eprintln!("❌ Failed to parse UUID from string '{}': {}", player_id_string, e);
                }
            }
        }
    }
}