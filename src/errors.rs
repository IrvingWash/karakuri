use std::any::type_name;

use kec::Entity;

pub fn panic_queried<T: ?Sized>(entity: &Entity) -> ! {
    klogger::terminate(&format!(
        "Entity {} didn't have {}, though was queried for it.",
        entity.unique_id(),
        type_name::<T>()
    ))
}

pub fn panic_uninitialized_sprite(name: &str) -> ! {
    klogger::terminate(&format!(
        "Sprite doesn't have `{}` though it should have been populated.",
        name
    ))
}

pub fn panic_uninitialized_collider(name: &str) -> ! {
    klogger::terminate(&format!(
        "Collider doesn't have `{}` though it should have been populated.",
        name
    ))
}

pub fn panic_not_loaded_texture(name: &str) -> ! {
    klogger::terminate(&format!("Tried to use not loaded texture {}.", name))
}
