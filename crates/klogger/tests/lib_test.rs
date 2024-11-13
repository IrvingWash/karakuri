#[test]
fn test_printing() {
    klogger::info("Test");
    klogger::warn("Test");
    klogger::error("Test");
}

#[test]
#[should_panic]
fn test_panicking() {
    klogger::terminate("Test");
}
