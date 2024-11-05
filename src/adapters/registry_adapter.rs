use std::cell::{Ref, RefMut};

use kec::{Entity, Registry};

#[derive(Debug)]
pub struct RegistryAdapter<'a> {
    registry: &'a Registry,
}

impl<'a> RegistryAdapter<'a> {
    pub fn new(registry: &'a Registry) -> Self {
        Self { registry }
    }

    pub fn is_alive(&self, entity: &Entity) -> bool {
        self.registry.is_alive(entity)
    }

    pub fn get_component<T: 'static>(&self, entity: &Entity) -> Option<Ref<'_, T>> {
        self.registry.get_component::<T>(entity)
    }

    pub fn get_component_mut<T: 'static>(&self, entity: &Entity) -> Option<RefMut<'_, T>> {
        self.registry.get_component_mut::<T>(entity)
    }

    pub fn find_entity<T: 'static + PartialEq>(&self, component_to_find: &T) -> Option<Entity> {
        self.registry.find_entity(component_to_find)
    }
}
