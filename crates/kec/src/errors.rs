use std::any::type_name;

#[inline]
pub fn panic_registered_without_id<T>() -> ! {
    klogger::terminate(&format!(
        "Component {} should have been already registered, but doesn't have an id.",
        type_name::<T>()
    ))
}

#[cfg(test)]
mod errors_tests {
    use super::panic_registered_without_id;

    #[test]
    #[should_panic]
    fn test_panic_registered_without_id() {
        panic_registered_without_id::<f64>();
    }
}
