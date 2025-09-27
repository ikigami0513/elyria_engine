use crate::{core::{input::InputHandler, time::Time}, graphics::managers::{AnimationManager, SpritesheetManager}, world::world::World};

#[allow(dead_code)]
pub struct FrameContext<'a> {
    pub time: &'a Time,
    pub input: &'a InputHandler,
    pub world: &'a mut World,
    pub spritesheet_manager: &'a mut SpritesheetManager,
    pub animation_manager: &'a mut AnimationManager
}
