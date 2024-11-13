pub fn panic_checked_circle_unwrap() -> ! {
    klogger::terminate("Couldn't unwrap a circle though the check for circle has been passed.")
}

pub fn panic_checked_polygon_unwrap() -> ! {
    klogger::terminate("Couldn't unwrap a polygon though the check for circle has been passed.")
}
