use crate::glutils::shader::Shader;
use crate::world::entity::Entity;
use std::rc::Rc;
use std::cell::RefCell;

#[allow(unused_variables)]
pub trait Component {
    fn update(&self, owner: &Rc<RefCell<Entity>>);
    fn render(&self, owner: &Rc<RefCell<Entity>>, shader: &Shader) {}
}
