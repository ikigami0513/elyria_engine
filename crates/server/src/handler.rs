use common::message::Message;
use engine::world::world::World;
use tokio::net::TcpStream;
use async_trait::async_trait;

#[allow(dead_code)]
pub struct HandlerContext<'a> {
    pub message: &'a Message,
    pub world: &'a mut World,
    pub socket: &'a mut TcpStream
}

#[async_trait]
pub trait Handler {
    async fn handle<'ctx>(&self, ctx: HandlerContext<'ctx>);
}
