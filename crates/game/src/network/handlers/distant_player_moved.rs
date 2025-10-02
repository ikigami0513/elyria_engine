use cgmath::vec3;
use engine::{core::frame_context::FrameContext, graphics::{animation::AnimationComponent, sprite::SpriteCreator}, world::components::TransformComponent};
use uuid::Uuid;

use crate::{network::{event::NetworkEvent, handlers::handler::Handler}, player::{Direction, DistantPlayerComponent, State}};

pub struct DistantPlayerMovedHandler;

impl Handler for DistantPlayerMovedHandler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent) {
        if let Some(player_id_string) = event.data.get("player_id") {
            match Uuid::parse_str(player_id_string) {
                Ok(player_id) => {
                    if let Some(distant_player_comps) = ctx.world.get_components_mut::<DistantPlayerComponent>() {
                        for (_entity, distant_player_comp) in distant_player_comps {
                            if distant_player_comp.player_id == player_id {
                                let x = event.data.get("x").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                                let y = event.data.get("y").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);
                                let z = event.data.get("z").and_then(|v| v.parse::<f32>().ok()).unwrap_or(0.0);

                                distant_player_comp.target_position = Some(vec3(x, y, z));
                                return;
                            }
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