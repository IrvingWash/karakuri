use std::any::type_name;

use kec::Entity;

pub fn panic_queried<T: ?Sized>(entity: &Entity) -> ! {
    panic!(
        "Entity {} didn't have {}, though was queried for it.",
        entity.id(),
        type_name::<T>()
    )
}

pub fn panic_uninitialized_sprite(name: &str) -> ! {
    panic!(
        "Sprite doesn't have `{}` though it should have been populated.",
        name
    );
}

pub fn panic_uninitialized_collider(name: &str) -> ! {
    panic!(
        "Collider doesn't have `{}` though it should have been populated.",
        name
    )
}

pub fn panic_not_loaded_texture(name: &str) -> ! {
    panic!("Tried to use not loaded texture {}.", name)
}
