use hashbrown::HashMap;
use std::any::{Any, TypeId};
use crate::world::components::Component;
use crate::world::entity::Entity;

pub struct World {
    next_entity_id: u32,
    components: HashMap<TypeId, Box<dyn Any>>
}

impl World {
    pub fn new() -> Self {
        World {
            next_entity_id: 0,
            components: HashMap::new()
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity = Entity(self.next_entity_id);
        self.next_entity_id += 1;
        entity
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let components_of_type = self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));

        let components_of_type_map = components_of_type
            .downcast_mut::<HashMap<Entity, T>>()
            .expect("Erreur de downcast de composant");

        components_of_type_map.insert(entity, component);
    }

    pub fn get_components<T: Component + 'static>(&self) -> Option<&HashMap<Entity, T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|box_map| box_map.downcast_ref::<HashMap<Entity, T>>())
    }

    pub fn get_components_mut<T: Component + 'static>(&mut self) -> Option<&mut HashMap<Entity, T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|box_map| box_map.downcast_mut::<HashMap<Entity, T>>())
    }
}