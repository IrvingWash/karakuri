use std::{
    any::{type_name, Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{errors::panic_registered_without_id, Entity, Query, Signature};

type Orra = Option<Rc<RefCell<dyn Any>>>;

const NO_SIGNATURE_MESSAGE: &str = "Signature should have been already created";

#[derive(Debug, Default)]
pub struct Registry {
    next_unique_id: usize,
    entities: Vec<Option<Entity>>,
    components: HashMap<TypeId, Vec<Orra>>,
    free_ids: HashSet<usize>,
    component_ids: HashMap<TypeId, usize>,
    entity_signatures: HashMap<Entity, Signature>,
}

impl Registry {
    #[inline]
    pub fn new() -> Self {
        Self {
            next_unique_id: 0,
            components: HashMap::with_capacity(64),
            free_ids: HashSet::with_capacity(64),
            component_ids: HashMap::with_capacity(64),
            entity_signatures: HashMap::with_capacity(64),
            entities: Vec::with_capacity(64),
        }
    }

    #[inline]
    pub fn create_entity(&mut self) -> Entity {
        let unique_id = self.next_unique_id;
        self.next_unique_id += 1;

        match self
            .free_ids
            .iter()
            .next()
            .cloned()
            .and_then(|x| self.free_ids.take(&x))
        {
            Some(id) => {
                let entity = Entity::new(id, unique_id);

                self.entity_signatures
                    .insert(entity.clone(), Signature::new());

                self.entities[id] = Some(entity.clone());

                entity
            }
            None => {
                let id = self.entities.len();

                for component_vec in self.components.values_mut() {
                    component_vec.push(None);
                }

                let entity = Entity::new(id, unique_id);

                self.entity_signatures
                    .insert(entity.clone(), Signature::new());

                self.entities.push(Some(entity.clone()));

                entity
            }
        }
    }

    #[inline]
    pub fn is_alive(&self, entity: &Entity) -> bool {
        if let Some(held_entity) = &self.entities[entity.key()] {
            return held_entity.unique_id() == entity.unique_id();
        }

        false
    }

    #[inline]
    pub const fn component_ids(&self) -> &HashMap<TypeId, usize> {
        &self.component_ids
    }

    #[inline]
    pub const fn entity_signatures(&self) -> &HashMap<Entity, Signature> {
        &self.entity_signatures
    }

    #[inline]
    pub fn remove_entity(&mut self, entity: &Entity) {
        let entity_key = entity.key();

        for component_vec in self.components.values_mut() {
            component_vec[entity_key] = None;
        }

        self.entity_signatures.remove_entry(entity);

        self.entities[entity_key] = None;

        self.free_ids.insert(entity_key);
    }

    #[inline]
    pub fn register_component<T: Any + ?Sized>(&mut self) {
        let component_type = TypeId::of::<T>();

        if self.components.contains_key(&component_type) {
            return;
        }

        let component_id = self.component_ids.len();
        self.component_ids.insert(component_type, component_id);

        let new_component_vec: Vec<Orra> = vec![None; self.entities.len()];
        self.components.insert(component_type, new_component_vec);
    }

    #[inline]
    pub fn add_dyn_component<T: Any + ?Sized>(&mut self, entity: &Entity, component: Box<T>) {
        self.register_component::<T>();

        let id = entity.key();
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
            .get_mut(entity)
            .expect(NO_SIGNATURE_MESSAGE)
            .set(*component_id);
    }

    #[inline]
    pub fn add_component<T: Any>(&mut self, entity: &Entity, component: T) {
        self.register_component::<T>();

        let entity_key = entity.key();
        let wrapped_component: Orra = Some(Rc::new(RefCell::new(component)));
        let component_type = TypeId::of::<T>();

        let component_id = self
            .component_ids
            .get(&component_type)
            .unwrap_or_else(|| panic_registered_without_id::<T>());

        self.components.get_mut(&component_type).unwrap_or_else(|| {
            klogger::terminate(&format!(
                "Component {} should have been already registered, but doesn't have a component vec.",
                type_name::<T>()
            ))
        })[entity_key] = wrapped_component;

        self.entity_signatures
            .get_mut(entity)
            .expect(NO_SIGNATURE_MESSAGE)
            .set(*component_id);
    }

    #[inline]
    pub fn get_dyn_component<T: Any + ?Sized>(&self, entity: &Entity) -> Option<Ref<Box<T>>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.key()] {
                None => return None,
                Some(component) => return Self::borrow_downcast::<Box<T>>(component),
            }
        }

        None
    }

    #[inline]
    pub fn get_component<T: Any>(&self, entity: &Entity) -> Option<Ref<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.key()] {
                None => return None,
                Some(component) => return Self::borrow_downcast::<T>(component),
            }
        }

        None
    }

    #[inline]
    pub fn get_dyn_component_mut<T: Any + ?Sized>(
        &self,
        entity: &Entity,
    ) -> Option<RefMut<Box<T>>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.key()] {
                None => return None,
                Some(component) => return Self::borrow_downcast_mut::<Box<T>>(component),
            }
        }

        None
    }

    #[inline]
    pub fn get_component_mut<T: Any>(&self, entity: &Entity) -> Option<RefMut<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[entity.key()] {
                None => return None,
                Some(component) => return Self::borrow_downcast_mut::<T>(component),
            }
        }

        None
    }

    #[inline]
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

    #[inline]
    pub fn get_dyn_component_vec<T: Any + ?Sized>(&self) -> Vec<Option<Ref<Box<T>>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => Vec::new(),
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast::<Box<T>>(c),
                })
                .collect(),
        }
    }

    #[inline]
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

    #[inline]
    pub fn get_dyn_component_vec_mut<T: Any + ?Sized>(&self) -> Vec<Option<RefMut<Box<T>>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => Vec::new(),
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast_mut::<Box<T>>(c),
                })
                .collect(),
        }
    }

    #[inline]
    pub fn find_entity<T: Any + PartialEq>(&self, component_to_find: &T) -> Option<Entity> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            let key = component_vec.iter().position(|component| match component {
                None => false,
                Some(component) => Self::borrow_downcast::<T>(component)
                    .unwrap_or_else(|| {
                        klogger::terminate(&format!(
                            "Failed to downcast component {} although it was found",
                            type_name::<T>()
                        ))
                    })
                    .eq(component_to_find),
            });

            match key {
                None => return None,
                Some(id) => {
                    return self
                        .entity_signatures()
                        .keys()
                        .find(|entity| entity.key() == id)
                        .cloned()
                }
            }
        }

        None
    }

    #[inline]
    pub fn query(&mut self) -> Query {
        Query::new(self)
    }

    #[inline]
    pub fn has<T: Any>(&self) -> bool {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            return component_vec.iter().any(|component| component.is_some());
        }

        false
    }

    fn borrow_downcast<T: Any>(cell: &Rc<RefCell<dyn Any>>) -> Option<Ref<T>> {
        let r = cell.borrow();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(Ref::map(r, |x| {
                x.downcast_ref::<T>().unwrap_or_else(|| {
                    klogger::terminate(&format!(
                        "Failed to downcast component {}",
                        type_name::<T>()
                    ))
                })
            }))
        } else {
            None
        }
    }

    fn borrow_downcast_mut<T: Any>(cell: &RefCell<dyn Any>) -> Option<RefMut<T>> {
        let r = cell.borrow_mut();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(RefMut::map(r, |x| {
                x.downcast_mut::<T>().unwrap_or_else(|| {
                    klogger::terminate(&format!(
                        "Failed to mutably downcast component {}",
                        type_name::<T>()
                    ))
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
    #[derive(Debug, PartialEq)]
    struct Tag(&'static str);

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

        assert_eq!(registry.entities.len(), 2);
        assert_eq!(registry.components.len(), 2);

        assert!(registry.get_component::<Health>(&eggman).is_some());
        assert!(registry.get_component::<Health>(&sonic).is_some());

        assert!(registry.get_component::<Speed>(&eggman).is_none());
        assert!(registry.get_component::<Speed>(&sonic).is_some());

        registry.remove_entity(&eggman);

        assert_eq!(registry.entities.len(), 2);
        assert_eq!(registry.components.len(), 2);
        assert_eq!(registry.free_ids.len(), 1);
        assert!(registry.free_ids.contains(&0));

        assert!(registry.get_component::<Health>(&eggman).is_none());
        assert!(registry.get_component::<Health>(&sonic).is_some());

        assert!(registry.get_component::<Speed>(&eggman).is_none());
        assert!(registry.get_component::<Speed>(&sonic).is_some());

        let tails = registry.create_entity();
        registry.add_component(&tails, Health(1));
        registry.add_component(&tails, Speed(15));

        assert_eq!(registry.entities.len(), 2);
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

        registry.add_dyn_component(&tails, flyer);

        registry
            .get_dyn_component::<dyn Flyer>(&tails)
            .unwrap()
            .fly();
        registry
            .get_dyn_component::<dyn Flyer>(&tails)
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

    #[test]
    fn test_is_alive() {
        let mut registry = Registry::new();

        let entity = registry.create_entity();

        assert!(registry.is_alive(&entity));

        registry.remove_entity(&entity);

        assert!(!registry.is_alive(&entity));
    }

    #[test]
    fn test_find_entity() {
        let mut registry = Registry::new();

        let entity = registry.create_entity();
        registry.add_component(&entity, Tag("entity"));

        assert!(registry.find_entity(&Tag("entity")).is_some());
        assert!(registry.find_entity(&Tag("not entity")).is_none());
    }

    #[test]
    fn test_has() {
        let mut registry = Registry::new();

        assert!(!registry.has::<Tag>());

        let entity = registry.create_entity();
        registry.add_component(&entity, Tag("entity"));

        assert!(registry.has::<Tag>());
    }
}
