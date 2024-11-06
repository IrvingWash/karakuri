use std::any::{Any, TypeId};

use crate::{errors::panic_registered_without_id, Entity, Registry, Signature};

pub struct Query<'a> {
    registry: &'a mut Registry,
    queried_component_ids: Vec<usize>,
}

impl<'a> Query<'a> {
    #[inline]
    pub fn new(registry: &'a mut Registry) -> Self {
        Self {
            registry,
            queried_component_ids: Vec::with_capacity(64),
        }
    }

    #[inline]
    pub fn build(mut self) -> Vec<Entity> {
        self.queried_component_ids.dedup();

        let signature = Signature::from(self.queried_component_ids);

        if signature.is_empty() {
            return Vec::new();
        }

        self.registry
            .entity_signatures()
            .iter()
            .filter_map(|(key, value)| {
                if value.is_superset(&signature) {
                    return Some(key.clone());
                }

                None
            })
            .collect()
    }

    #[inline]
    pub fn with_component<T: Any>(mut self) -> Query<'a> {
        let component_type = TypeId::of::<T>();

        match self.registry.component_ids().get(&component_type) {
            Some(component_id) => {
                self.queried_component_ids.push(*component_id);
            }
            None => {
                self.registry.register_component::<T>();

                self.queried_component_ids.push(
                    *self
                        .registry
                        .component_ids()
                        .get(&component_type)
                        .unwrap_or_else(|| panic_registered_without_id::<T>()),
                )
            }
        }

        self
    }
}
