const KARAKURI: &str = "karakuri";
const ANSI_START: &str = "\x1b[0;";
const ANSI_END: &str = "\x1b[0m";
const BLUE: &str = "34m";
const RED: &str = "31m";
const CYAN: &str = "35m";

#[inline]
pub fn info(message: &str) {
    print(message, BLUE)
}

#[inline]
pub fn warn(message: &str) {
    print(message, CYAN)
}

#[inline]
pub fn error(message: &str) {
    print(message, RED);
}

#[inline]
pub fn terminate(message: &str) -> ! {
    panic!("{KARAKURI} terminate: {ANSI_START}{RED}{message}{ANSI_END}");
}

fn print(message: &str, color: &'static str) {
    let value = format!("{KARAKURI} error: {ANSI_START}{color}{message}{ANSI_END}");

    println!("{value}");
}
