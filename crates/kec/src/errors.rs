use std::any::type_name;

#[inline]
pub fn panic_registered_without_id<T: ?Sized>() -> ! {
    klogger::terminate(&format!(
        "Component {} should have been already registered, but doesn't have an id.",
        type_name::<T>()
    ))
}
