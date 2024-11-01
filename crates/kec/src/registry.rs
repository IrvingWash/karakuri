use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use crate::{errors::panic_registered_without_id, Entity, Query, Signature};

type Orra = Option<Rc<RefCell<dyn Any>>>;

const NO_SIGNATURE_MESSAGE: &str = "Signature should have been already created";

#[derive(Debug, Default)]
pub struct Registry {
    entity_count: usize,
    components: HashMap<TypeId, Vec<Orra>>,
    free_ids: Vec<usize>,
    component_ids: HashMap<TypeId, usize>,
    entity_signatures: HashMap<usize, Signature>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            entity_count: 0,
            components: HashMap::with_capacity(64),
            free_ids: Vec::with_capacity(64),
            component_ids: HashMap::with_capacity(64),
            entity_signatures: HashMap::with_capacity(64),
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        match self.free_ids.pop() {
            Some(id) => Entity::new(id),
            None => {
                let id = self.entity_count;
                self.entity_count += 1;

                for component_vec in self.components.values_mut() {
                    component_vec.push(None);
                }

                self.entity_signatures.insert(id, Signature::new());

                Entity::new(id)
            }
        }
    }

    pub fn component_ids(&self) -> &HashMap<TypeId, usize> {
        &self.component_ids
    }

    pub fn entity_signatures(&self) -> &HashMap<usize, Signature> {
        &self.entity_signatures
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        let id = entity.id();

        for component_vec in self.components.values_mut() {
            component_vec[id] = None;
        }

        self.entity_signatures
            .get_mut(&id)
            .expect(NO_SIGNATURE_MESSAGE)
            .reset();

        self.free_ids.push(id);
    }

    pub fn register_component<T: Any>(&mut self) {
        let component_type = TypeId::of::<T>();

        if self.components.contains_key(&component_type) {
            return;
        }

        let component_id = self.component_ids.len();
        self.component_ids.insert(component_type, component_id);

        let new_component_vec: Vec<Orra> = vec![None; self.entity_count];
        self.components.insert(component_type, new_component_vec);
    }

    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) {
        self.register_component::<T>();

        let id = entity.id();
        let wrapped_component: Orra = Some(Rc::new(RefCell::new(component)));
        let component_type = TypeId::of::<T>();

        let component_id = self
            .component_ids
            .get(&component_type)
            .unwrap_or_else(|| panic_registered_without_id::<T>());

        self.components.get_mut(&component_type).unwrap_or_else(|| {
            panic!(
                "Component {} should have been already registered, but doesn't have a component vec.",
                type_name::<T>()
            )
        })[id] = wrapped_component;

        self.entity_signatures
            .get_mut(&id)
            .expect(NO_SIGNATURE_MESSAGE)
            .set(*component_id);
    }

    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<Ref<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.id()] {
                None => return None,
                Some(component) => return Self::borrow_downcast::<T>(component),
            }
        }

        None
    }

    pub fn get_component_mut<T: Any>(&self, entity: &Entity) -> Option<RefMut<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.id()] {
                None => return None,
                Some(component) => return Self::borrow_downcast_mut::<T>(component),
            }
        }

        None
    }

    pub fn get_component_vec<T: Any>(&self) -> Vec<Option<Ref<T>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => Vec::new(),
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast::<T>(c),
                })
                .collect(),
        }
    }

    pub fn get_component_vec_mut<T: Any>(&self) -> Vec<Option<RefMut<T>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => Vec::new(),
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast_mut::<T>(c),
                })
                .collect(),
        }
    }

    pub fn find_entity<T: Any + PartialEq>(&self, component_to_find: T) -> Option<Entity> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            let id = component_vec.iter().position(|component| match component {
                None => false,
                Some(component) => Self::borrow_downcast::<T>(component)
                    .unwrap_or_else(|| {
                        panic!(
                            "Failed to downcast component {} although it was found",
                            type_name::<T>()
                        )
                    })
                    .eq(&component_to_find),
            });

            return id.map(Entity::new);
        }

        None
    }

    pub fn query(&mut self) -> Query {
        Query::new(self)
    }

    fn borrow_downcast<T: Any>(cell: &Rc<RefCell<dyn Any>>) -> Option<Ref<T>> {
        let r = cell.borrow();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(Ref::map(r, |x| {
                x.downcast_ref::<T>()
                    .unwrap_or_else(|| panic!("Failed to downcast component {}", type_name::<T>()))
            }))
        } else {
            None
        }
    }

    pub fn borrow_downcast_mut<T: Any>(cell: &RefCell<dyn Any>) -> Option<RefMut<T>> {
        let r = cell.borrow_mut();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(RefMut::map(r, |x| {
                x.downcast_mut::<T>().unwrap_or_else(|| {
                    panic!("Failed to mutably downcast component {}", type_name::<T>())
                })
            }))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod world_tests {
    use std::any::Any;

    use super::Registry;

    #[derive(Debug)]
    struct Health(u32);
    #[derive(Debug)]
    struct Speed(u32);

    trait Flyer {
        fn fly(&self);
        fn as_any(&self) -> &dyn Any;
    }

    #[test]
    fn test_entity_lifecycle() {
        let mut registry = Registry::new();

        let eggman = registry.create_entity();
        registry.add_component(&eggman, Health(500));

        let sonic = registry.create_entity();
        registry.add_component(&sonic, Health(1));
        registry.add_component(&sonic, Speed(30));

        assert_eq!(registry.entity_count, 2);
        assert_eq!(registry.components.len(), 2);

        assert!(registry.get_component::<Health>(&eggman).is_some());
        assert!(registry.get_component::<Health>(&sonic).is_some());

        assert!(registry.get_component::<Speed>(&eggman).is_none());
        assert!(registry.get_component::<Speed>(&sonic).is_some());

        registry.remove_entity(&eggman);

        assert_eq!(registry.entity_count, 2);
        assert_eq!(registry.components.len(), 2);
        assert_eq!(registry.free_ids.len(), 1);
        assert_eq!(registry.free_ids[0], 0);

        assert!(registry.get_component::<Health>(&eggman).is_none());
        assert!(registry.get_component::<Health>(&sonic).is_some());

        assert!(registry.get_component::<Speed>(&eggman).is_none());
        assert!(registry.get_component::<Speed>(&sonic).is_some());

        let tails = registry.create_entity();
        registry.add_component(&tails, Health(1));
        registry.add_component(&tails, Speed(15));

        assert_eq!(registry.entity_count, 2);
        assert_eq!(registry.components.len(), 2);
        assert_eq!(registry.free_ids.len(), 0);

        assert!(registry.get_component::<Health>(&sonic).is_some());
        assert!(registry.get_component::<Health>(&tails).is_some());

        assert!(registry.get_component::<Speed>(&sonic).is_some());
        assert!(registry.get_component::<Speed>(&tails).is_some());

        assert_eq!(registry.get_component::<Health>(&sonic).unwrap().0, 1);
        assert_eq!(registry.get_component::<Speed>(&sonic).unwrap().0, 30);

        assert_eq!(registry.get_component::<Health>(&tails).unwrap().0, 1);
        assert_eq!(registry.get_component::<Speed>(&tails).unwrap().0, 15);

        assert_eq!(registry.get_component_vec::<Health>().len(), 2);
        assert_eq!(registry.get_component_vec_mut::<Speed>().len(), 2);
    }

    #[test]
    fn test_trait_component() {
        let mut registry = Registry::new();

        struct Tails {
            name: String,
        }
        impl Tails {
            fn say_hi(&self) {
                println!("I am {}", self.name)
            }
        }
        impl Flyer for Tails {
            fn fly(&self) {
                println!("{} is flying", self.name)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        let tails = registry.create_entity();
        let flyer: Box<dyn Flyer> = Box::new(Tails {
            name: String::from("Tails"),
        });

        registry.add_component(&tails, flyer);

        registry
            .get_component::<Box<dyn Flyer>>(&tails)
            .unwrap()
            .fly();
        registry
            .get_component::<Box<dyn Flyer>>(&tails)
            .unwrap()
            .as_any()
            .downcast_ref::<Tails>()
            .unwrap()
            .say_hi();
    }

    #[test]
    fn test_get_component_vec() {
        let mut registry = Registry::new();

        let sonic = registry.create_entity();
        let tails = registry.create_entity();

        registry.add_component(&sonic, Health(100));
        registry.add_component(&tails, Speed(30));

        {
            let healths = registry.get_component_vec::<Health>();
            assert_eq!(healths.len(), 2);
            assert!(healths[0].is_some());
            assert!(healths[1].is_none());
        }

        let healths = registry.get_component_vec_mut::<Health>();
        assert_eq!(healths.len(), 2);
        assert!(healths[0].is_some());
        assert!(healths[1].is_none());
    }
}
