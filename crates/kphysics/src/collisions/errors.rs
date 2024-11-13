pub fn panic_checked_circle_unwrap() -> ! {
    klogger::terminate("Couldn't unwrap a circle though the check for circle has been passed.")
}

pub fn panic_checked_polygon_unwrap() -> ! {
    klogger::terminate("Couldn't unwrap a polygon though the check for circle has been passed.")
}

#[cfg(test)]
mod errors_tests {
    use super::{panic_checked_circle_unwrap, panic_checked_polygon_unwrap};

    #[test]
    #[should_panic]
    fn test_panic_checked_circle_unwrap() {
        panic_checked_circle_unwrap();
    }

    #[test]
    #[should_panic]
    fn test_panic_checked_polygon_unwrap() {
        panic_checked_polygon_unwrap()
    }
}
