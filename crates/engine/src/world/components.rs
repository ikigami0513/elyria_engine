use crate::{math::transform::Transform, world::entity::Entity};

#[allow(unused_variables)]
pub trait Component {}

#[derive(Clone, Copy)]
pub struct Parent(pub Entity);
impl Component for Parent {}

#[derive(Clone)]
pub struct TransformComponent {
    pub transform: Transform
}

impl TransformComponent {
    pub fn new() -> Self {
        TransformComponent {
            transform: Transform::new()
        }
    }
}

impl Component for TransformComponent {}
