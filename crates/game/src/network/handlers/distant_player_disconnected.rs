use uuid::Uuid;

use crate::{network::handlers::handler::Handler, player::DistantPlayerComponent};

pub struct DistantPlayerDisconnectedHandler;

impl Handler for DistantPlayerDisconnectedHandler {
    fn handle(&self, ctx: &mut engine::core::frame_context::FrameContext, event: crate::network::event::NetworkEvent) {
        if let Some(player_id_string) = event.data.get("player_id") {
            match Uuid::parse_str(player_id_string) {
                Ok(player_id) => {
                    if let Some(distant_player_comps) = ctx.world.get_components::<DistantPlayerComponent>() {
                        for (entity, distant_player_comp) in distant_player_comps {
                            if distant_player_comp.player_id == player_id {
                                ctx.world.remove_entity(*entity);
                                break;
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