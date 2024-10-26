use std::any::{Any, TypeId};

use crate::{Entity, Registry, Signature};

pub struct Query<'a> {
    registry: &'a mut Registry,
    component_ids: Vec<usize>,
}

impl<'a> Query<'a> {
    pub fn new(registry: &'a mut Registry) -> Self {
        Self {
            registry,
            component_ids: Vec::with_capacity(64),
        }
    }

    pub fn build(mut self) -> Vec<Entity> {
        self.component_ids.dedup();

        let signature = Signature::from(self.component_ids);

        if signature.is_empty() {
            return Vec::new();
        }

        self.registry
            .entity_signatures()
            .iter()
            .filter_map(|(key, value)| {
                if value.is_superset(&signature) {
                    return Some(Entity::new(*key));
                }

                None
            })
            .collect()
    }

    pub fn with_component<T: Any>(mut self) -> Query<'a> {
        if let Some(component_id) = self.registry.component_ids().get(&TypeId::of::<T>()) {
            self.component_ids.push(*component_id);
        }

        self
    }
}
