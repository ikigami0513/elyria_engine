use std::collections::HashMap;

use engine::{core::frame_context::FrameContext, world::system::System};
use tokio::sync::mpsc::Receiver;
use uuid::Uuid;

use crate::{gamestate::GameStateComponent, network::event::NetworkEvent};

pub trait Handler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent);
}

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
    }
}

pub struct NetworkEventSystem {
    receiver: Receiver<NetworkEvent>,
    pub handlers: HashMap<String, Box<dyn Handler>>
}

impl NetworkEventSystem {
    pub fn new(receiver: Receiver<NetworkEvent>) -> Self {
        Self { 
            receiver,
            handlers: HashMap::new()
        }
    }
}

impl System for NetworkEventSystem {
    fn update(&mut self, ctx: &mut FrameContext) {
        while let Ok(event) = self.receiver.try_recv() {
            if let Some(action) = event.data.get("action") {
                if let Some(handler) = self.handlers.get(action) {
                    handler.handle(ctx, event);
                }
                else {
                    eprintln!("Handler for action '{}' not found.", action);
                }
            }
            else {
                eprintln!("Key 'action' missing in the message");
            }
        }
    }
}
