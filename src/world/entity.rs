use std::collections::LinkedList;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::core::frame_context::FrameContext;
use crate::glutils::shader::Shader;
use crate::math::transform::Transform;
use crate::world::component::Component;

pub struct Entity {
    // Scene graph
    pub children: LinkedList<Rc<RefCell<Entity>>>,
    pub parent: Option<Weak<RefCell<Entity>>>,
    pub components: LinkedList<Box<dyn Component>>,

    // Space information
    pub transform: Transform
}

#[allow(dead_code)]
impl Entity {
    pub fn new() -> Self {
        Entity {
            children: LinkedList::new(),
            parent: None,
            components: LinkedList::new(),
            transform: Transform::new()
        }
    }

    pub fn add_component<T: Component + 'static>(&mut self, component: T) {
        self.components.push_back(Box::new(component));
    }

    pub fn add_child(parent: &Rc<RefCell<Entity>>) {
        let child = Rc::new(RefCell::new(Entity::new()));
        child.borrow_mut().parent = Some(Rc::downgrade(parent));
        parent.borrow_mut().children.push_back(child);
    }

    pub fn update(entity_rc: &Rc<RefCell<Entity>>, ctx: &FrameContext) {
        let mut entity_borrow = entity_rc.borrow_mut();

        // Si l'entité est "sale", forcez la mise à jour de sa transformation.
        if entity_borrow.transform.is_dirty() {
            if let Some(parent_rc) = entity_borrow.parent.as_ref().and_then(|p| p.upgrade()) {
                let parent_model_matrix = parent_rc.borrow().transform.get_model_matrix().clone();
                entity_borrow.transform.compute_model_matrix_with_parent(&parent_model_matrix);
            } else {
                entity_borrow.transform.compute_model_matrix();
            }

            // Mettre à jour les enfants après la mise à jour de l'entité actuelle.
            // On lâche l'emprunt mutable avant la boucle récursive.
            drop(entity_borrow); 
            
            for child_rc in entity_rc.borrow().children.iter() {
                Self::update(child_rc, ctx);
            }
            return;
        }

        // Si l'entité n'est pas "sale", mettez à jour les composants et les enfants.
        // On lâche l'emprunt mutable avant les boucles.
        drop(entity_borrow);
        
        for component in entity_rc.borrow().components.iter() {
            component.update(entity_rc, ctx);
        }

        for child_rc in entity_rc.borrow().children.iter() {
            Self::update(child_rc, ctx);
        }
    }
    
    // La méthode force_update est supprimée et sa logique est fusionnée dans update.
    
    pub fn render(entity_rc: &Rc<RefCell<Entity>>, shader: &Shader) {
        let entity_borrow = entity_rc.borrow();

        for component in &entity_borrow.components {
            component.render(entity_rc, shader);
        }

        for child_rc in &entity_borrow.children {
            Self::render(child_rc, shader);
        }
    }
}
