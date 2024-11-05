const KARAKURI: &str = "karakuri";
const ANSI_START: &str = "\x1b[0;";
const ANSI_END: &str = "\x1b[0m";
const BLUE: &str = "34m";
const RED: &str = "31m";
const CYAN: &str = "35m";

pub fn info(message: &str) {
    println!("{KARAKURI} info: {ANSI_START}{BLUE}{message}{ANSI_END}");
}

pub fn warn(message: &str) {
    println!("{KARAKURI} warn: {ANSI_START}{CYAN}{message}{ANSI_END}");
}

pub fn error(message: &str) {
    println!("{KARAKURI} error: {ANSI_START}{RED}{message}{ANSI_END}");
}

pub fn terminate(message: &str) -> ! {
    panic!("{KARAKURI} terminate: {ANSI_START}{RED}{message}{ANSI_END}");
}
