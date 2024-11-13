use std::any::type_name;

use kec::Entity;

#[inline]
pub fn panic_queried<T: ?Sized>(entity: &Entity) -> ! {
    klogger::terminate(&format!(
        "Entity {} didn't have {}, though was queried for it.",
        entity.unique_id(),
        type_name::<T>()
    ))
}

#[inline]
pub fn panic_uninitialized_sprite(name: &str) -> ! {
    klogger::terminate(&format!(
        "Sprite doesn't have `{}` though it should have been populated.",
        name
    ))
}

#[inline]
pub fn panic_uninitialized_collider(name: &str) -> ! {
    klogger::terminate(&format!(
        "Collider doesn't have `{}` though it should have been populated.",
        name
    ))
}

#[inline]
pub fn panic_not_loaded_texture(name: &str) -> ! {
    klogger::terminate(&format!("Tried to use not loaded texture {}.", name))
}

#[cfg(test)]
mod errors_tests {
    use kec::Entity;

    use super::{
        panic_not_loaded_texture, panic_queried, panic_uninitialized_collider,
        panic_uninitialized_sprite,
    };

    #[test]
    #[should_panic]
    fn test_panic_queried() {
        panic_queried::<f64>(&Entity::new(1, 1));
    }

    #[test]
    #[should_panic]
    fn test_uninitialized_sprite() {
        panic_uninitialized_sprite("Test");
    }

    #[test]
    #[should_panic]
    fn test_uninitialized_collider() {
        panic_uninitialized_collider("Test");
    }

    #[test]
    #[should_panic]
    pub fn test_panic_not_loaded_texture() {
        panic_not_loaded_texture("Test");
    }
}
