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

    pub fn register_component<T: Component + 'static>(&mut self) {
        let type_id = TypeId::of::<T>();
        self.components.entry(type_id).or_insert_with(|| Box::new(HashMap::<Entity, T>::new()));
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
            .and_then(|box_map| box_map.downcast_ref::<HashMap<Entity, T>>())
    }

    pub fn get_components_mut<T: Component + 'static>(&mut self) -> Option<&mut HashMap<Entity, T>> {
        let type_id = TypeId::of::<T>();
        self.components
            .get_mut(&type_id)
            .and_then(|box_map| box_map.downcast_mut::<HashMap<Entity, T>>())
    }

    pub fn get_components_mut_pair<T, U>(&mut self, entity: Entity) -> Option<(&mut T, &mut U)>
    where
        T: Component + 'static,
        U: Component + 'static,
    {
        let type_id_t = TypeId::of::<T>();
        let type_id_u = TypeId::of::<U>();

        // La vérification de sécurité reste cruciale.
        if type_id_t == type_id_u {
            return None;
        }

        // On obtient un pointeur brut vers la HashMap AVANT de créer le moindre emprunt.
        let components_ptr: *mut _ = &mut self.components;

        // Tout se passe maintenant dans un seul bloc `unsafe`.
        // C'est notre contrat avec le compilateur : nous garantissons que les opérations ici sont sûres.
        unsafe {
            // On déréférence le pointeur pour obtenir le premier pool de composants.
            let map_t_box = (*components_ptr).get_mut(&type_id_t)?;
            
            // On déréférence le MÊME pointeur une seconde fois pour le deuxième pool.
            // Le compilateur l'autorise car nous sommes en mode `unsafe`.
            let map_u_box = (*components_ptr).get_mut(&type_id_u)?;

            // Le reste de la logique est identique.
            let map_t = map_t_box.downcast_mut::<HashMap<Entity, T>>()?;
            let map_u = map_u_box.downcast_mut::<HashMap<Entity, U>>()?;
            
            let comp_t = map_t.get_mut(&entity)?;
            let comp_u = map_u.get_mut(&entity)?;

            Some((comp_t, comp_u))
        }
    }
}