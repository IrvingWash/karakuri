use std::any::type_name;

use kec::Entity;

pub fn panic_queried<T: ?Sized>(entity: Entity) -> ! {
    panic!(
        "Entity {} didn't have {}, though was queried for it.",
        entity.id(),
        type_name::<T>()
    )
}
