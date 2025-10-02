use engine::core::frame_context::FrameContext;
use crate::network::event::NetworkEvent;

pub trait Handler {
    fn handle(&self, ctx: &mut FrameContext, event: NetworkEvent);
}