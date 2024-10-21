use std::{
    any::{Any, TypeId},
    cell::{Ref, RefCell, RefMut},
    collections::HashMap,
    rc::Rc,
};

use super::EntityId;

type Orra = Option<Rc<RefCell<dyn Any>>>;

#[derive(Debug)]
pub struct World {
    entities: Vec<EntityId>,
    components: HashMap<TypeId, Vec<Orra>>,

    free_ids: Vec<EntityId>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: Vec::with_capacity(1000),
            components: HashMap::new(),

            free_ids: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> EntityId {
        match self.free_ids.pop() {
            Some(id) => id,
            None => {
                let id = self.entities.len();
                self.entities.push(id);

                for component_vec in self.components.values_mut() {
                    component_vec.push(None);
                }

                id
            }
        }
    }

    pub fn remove_entity(&mut self, id: EntityId) {
        for component_vec in self.components.values_mut() {
            component_vec[id] = None;
        }

        self.free_ids.push(id);
    }

    pub fn add_component<T: Any>(&mut self, id: EntityId, component: T) {
        let wrapped_component: Orra = Some(Rc::new(RefCell::new(component)));
        let component_type = TypeId::of::<T>();

        match self.components.get_mut(&component_type) {
            Some(component_vec) => component_vec[id] = wrapped_component,
            None => {
                let mut new_component_vec: Vec<Orra> = vec![None; self.entities.len()];
                new_component_vec[id] = wrapped_component;

                self.components.insert(component_type, new_component_vec);
            }
        }
    }

    pub fn get_component<T: Any>(&mut self, id: EntityId) -> Option<Ref<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[id] {
                None => return None,
                Some(component) => return Self::borrow_downcast::<T>(component),
            }
        }

        None
    }

    pub fn get_component_mut<T: Any>(&mut self, id: EntityId) -> Option<RefMut<T>> {
        if let Some(component_vec) = self.components.get(&TypeId::of::<T>()) {
            match &component_vec[id] {
                None => return None,
                Some(component) => return Self::borrow_downcast_mut::<T>(component),
            }
        }

        None
    }

    pub fn get_component_vec<T: Any>(&mut self) -> Option<Vec<Ref<T>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => None,
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast::<T>(c),
                })
                .collect(),
        }
    }

    pub fn get_component_vec_mut<T: Any>(&mut self) -> Option<Vec<RefMut<T>>> {
        match self.components.get(&TypeId::of::<T>()) {
            None => None,
            Some(component_vec) => component_vec
                .iter()
                .map(|c| match c {
                    None => None,
                    Some(c) => Self::borrow_downcast_mut::<T>(c),
                })
                .collect(),
        }
    }

    fn borrow_downcast<T: Any>(cell: &Rc<RefCell<dyn Any>>) -> Option<Ref<T>> {
        let r = cell.borrow();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(Ref::map(r, |x| x.downcast_ref::<T>().unwrap()))
        } else {
            None
        }
    }

    pub fn borrow_downcast_mut<T: Any>(cell: &RefCell<dyn Any>) -> Option<RefMut<T>> {
        let r = cell.borrow_mut();
        if (*r).type_id() == TypeId::of::<T>() {
            Some(RefMut::map(r, |x| x.downcast_mut::<T>().unwrap()))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod world_tests {
    use std::any::Any;

    use super::World;

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
        let mut world = World::new();

        let eggman = world.create_entity();
        world.add_component(eggman, Health(500));

        let sonic = world.create_entity();
        world.add_component(sonic, Health(1));
        world.add_component(sonic, Speed(30));

        assert_eq!(world.entities.len(), 2);
        assert_eq!(world.components.len(), 2);

        assert!(world.get_component::<Health>(eggman).is_some());
        assert!(world.get_component::<Health>(sonic).is_some());

        assert!(world.get_component::<Speed>(eggman).is_none());
        assert!(world.get_component::<Speed>(sonic).is_some());

        world.remove_entity(eggman);

        assert_eq!(world.entities.len(), 2);
        assert_eq!(world.components.len(), 2);
        assert_eq!(world.free_ids.len(), 1);
        assert_eq!(world.free_ids[0], 0);

        assert!(world.get_component::<Health>(eggman).is_none());
        assert!(world.get_component::<Health>(sonic).is_some());

        assert!(world.get_component::<Speed>(eggman).is_none());
        assert!(world.get_component::<Speed>(sonic).is_some());

        let tails = world.create_entity();
        world.add_component(tails, Health(1));
        world.add_component(tails, Speed(15));

        assert_eq!(world.entities.len(), 2);
        assert_eq!(world.components.len(), 2);
        assert_eq!(world.free_ids.len(), 0);

        assert!(world.get_component::<Health>(sonic).is_some());
        assert!(world.get_component::<Health>(tails).is_some());

        assert!(world.get_component::<Speed>(sonic).is_some());
        assert!(world.get_component::<Speed>(tails).is_some());

        assert_eq!(world.get_component::<Health>(sonic).unwrap().0, 1);
        assert_eq!(world.get_component::<Speed>(sonic).unwrap().0, 30);

        assert_eq!(world.get_component::<Health>(tails).unwrap().0, 1);
        assert_eq!(world.get_component::<Speed>(tails).unwrap().0, 15);

        assert_eq!(world.get_component_vec::<Health>().unwrap().len(), 2);
        assert_eq!(world.get_component_vec_mut::<Speed>().unwrap().len(), 2);
    }

    #[test]
    fn test_trait_component() {
        let mut world = World::new();

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

        let tails = world.create_entity();
        let flyer: Box<dyn Flyer> = Box::new(Tails {
            name: String::from("Tails"),
        });

        world.add_component(tails, flyer);

        world.get_component::<Box<dyn Flyer>>(tails).unwrap().fly();
        world
            .get_component::<Box<dyn Flyer>>(tails)
            .unwrap()
            .as_any()
            .downcast_ref::<Tails>()
            .unwrap()
            .say_hi();
    }
}
