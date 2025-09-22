use std::rc::Rc;
use std::cell::RefCell;
use crate::core::frame_context::FrameContext;
use crate::glutils::shader::Shader;
use crate::world::entity::Entity;

pub struct Scene {
    root: Rc<RefCell<Entity>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            root: Rc::new(RefCell::new(Entity::new()))
        }
    }

    pub fn get_root(&self) -> Rc<RefCell<Entity>> {
        self.root.clone()
    }

    pub fn update(&mut self, ctx: &FrameContext) {
        Entity::update(&self.root, ctx);
    }

    pub fn render(&mut self, shader: &Shader) {
        Entity::render(&self.root, shader);
    }
}
