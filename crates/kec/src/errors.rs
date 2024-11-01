use std::any::type_name;

pub fn panic_registered_without_id<T>() -> ! {
    panic!(
        "Component {} should have been already registered, but doesn't have an id.",
        type_name::<T>()
    );
}
