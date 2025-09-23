use crate::{core::{input::InputHandler, time::Time}, world::world::World};

#[allow(dead_code)]
pub struct FrameContext<'a> {
    pub time: &'a Time,
    pub input: &'a InputHandler,
    pub world: &'a mut World
}
