use hashbrown::HashMap;
use std::any::{Any, TypeId};
use crate::world::components::Component;
use crate::world::entity::Entity;

trait IComponentMap: Any + Send + Sync {
    fn remove_entity(&mut self, entity: Entity);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Component + 'static + Send + Sync> IComponentMap for HashMap<Entity, T> {
    fn remove_entity(&mut self, entity: Entity) {
        self.remove(&entity);
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct World {
    next_entity_id: u32,
    components: HashMap<TypeId, Box<dyn IComponentMap>>
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
    
    pub fn remove_entity(&mut self, entity: Entity) {
        for component_map in self.components.values_mut() {
            component_map.remove_entity(entity);
        }
    }

    pub fn register_component<T: Component + 'static + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.entry(type_id).or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));
    }

    pub fn add_component<T: Component + 'static + Send + Sync>(&mut self, entity: Entity, component: T) {
        let type_id = TypeId::of::<T>();
        let components_of_type = self.components
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));

        let components_of_type_map = components_of_type
            .as_any_mut()
            .downcast_mut::<HashMap<Entity, T>>()
            .expect("Erreur de downcast de composant");

        components_of_type_map.insert(entity, component);
    }

    pub fn get_component<T: Component + 'static>(&self, entity: Entity) -> Option<&T> {
        self.get_components::<T>()
            .and_then(|components_map| components_map.get(&entity))
    }

    pub fn get_component_mut<T: Component + 'static>(&mut self, entity: Entity) -> Option<&mut T> {
        self.get_components_mut::<T>()
            .and_then(|components_map| components_map.get_mut(&entity))
    }

    pub fn get_components<T: Component + 'static>(&self) -> Option<&HashMap<Entity, T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get(&type_id)
            .and_then(|box_map| box_map.as_any().downcast_ref::<HashMap<Entity, T>>())
    }

    pub fn get_components_mut<T: Component + 'static>(&mut self) -> Option<&mut HashMap<Entity, T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|box_map| box_map.as_any_mut().downcast_mut::<HashMap<Entity, T>>())
    }
    
    pub fn get_components_mut_pair<T, U>(&mut self, entity: Entity) -> Option<(&mut T, &mut U)>
    where
        T: Component + 'static + Send + Sync,
        U: Component + 'static + Send + Sync,
    {
        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();

        if type_id_t == type_id_u {
            return None;
        }

        let components_ptr: *mut _ = &mut self.components;
        
        unsafe {
            let map_t_box = (*components_ptr).get_mut(&type_id_t)?;
            let map_u_box = (*components_ptr).get_mut(&type_id_u)?;
            
            let map_t = map_t_box.as_any_mut().downcast_mut::<HashMap<Entity, T>>()?;
            let map_u = map_u_box.as_any_mut().downcast_mut::<HashMap<Entity, U>>()?;
            
            let comp_t = map_t.get_mut(&entity)?;
            let comp_u = map_u.get_mut(&entity)?;

            Some((comp_t, comp_u))
        }
    }
}
