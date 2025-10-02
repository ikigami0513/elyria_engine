use std::collections::HashMap;

use engine::{core::frame_context::FrameContext, world::system::System};
use tokio::sync::mpsc::Receiver;

use crate::{network::{event::NetworkEvent, handlers::handler::Handler}};

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
