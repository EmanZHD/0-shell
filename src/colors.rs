// attributes
pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const RESET: &str = "\x1b[0m";

// Basic Colors
pub const RED: &str = "\x1b[91m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const BLUE: &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN: &str = "\x1b[36m";
pub const WHITE: &str = "\x1b[37m";
pub const GRAY: &str = "\x1b[90m";

// light colors
pub const LIGHT_GREEN: &str = "\x1b[92m";

// here to colorized
pub fn red(text: &str) -> String {
    format!("{}{}{}", RED, text, RESET)
}

pub fn green(text: &str) -> String {
    format!("{}{}{}", RESET, GREEN, text)
}

pub fn yellow(text: &str) -> String {
    format!("{}{}", YELLOW, text)
}

pub fn white(text: &str) -> String {
    format!("{}{}", WHITE, text)
}

pub fn bold_gray(text: &str) -> String {
    format!("{}{}{}", BOLD, GRAY, text)
}

pub fn blue(text: &str) -> String {
    format!("{}{}", BLUE, text)
}

pub fn cyan(text: &str) -> String {
    format!("{}{}", CYAN, text)
}

pub fn bold_red(text: &str) -> String {
    format!("{}{}{}", BOLD, RED, text)
}

pub fn bold_green(text: &str) -> String {
    format!("{}{}{}", BOLD, GREEN, text)
}

pub fn bold_blue(text: &str) -> String {
    format!("{}{}{}",BOLD, BLUE, text)
}

// dart errors red color w chi lakhor li bito