use std::{collections::HashMap, sync::Arc};

use common::message::Message;
use engine::world::world::World;
use tokio::{net::tcp::OwnedWriteHalf, sync::Mutex};
use async_trait::async_trait;
use uuid::Uuid;

#[allow(dead_code)]
pub struct MessageHandlerContext<'a> {
    pub message: &'a Message,
    pub world: &'a mut World,
    pub clients: &'a Mutex<HashMap<Uuid, Arc<Mutex<OwnedWriteHalf>>>>,
    pub current_player_id: Uuid
}

#[async_trait]
pub trait MessageHandler {
    async fn handle<'ctx>(&self, ctx: MessageHandlerContext<'ctx>);
}
