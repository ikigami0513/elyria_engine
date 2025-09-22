use crate::core::{input::InputHandler, time::Time};

#[allow(dead_code)]
pub struct FrameContext<'a> {
    pub time: &'a Time,
    pub input: &'a InputHandler
}
