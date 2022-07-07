#[macro_export]
macro_rules! plus {
    ($($arg:tt)+) => {{
        println!("{} {}", format!("{}::{}", $crate::colors::GREEN, $crate::colors::RESET), format!($($arg)+));
    }};
}

#[macro_export]
macro_rules! command {
    ($($arg:tt)+) => {{
        println!("{} {}", format!("{}[+]{}", $crate::colors::GREEN, $crate::colors::RESET), format!($($arg)+));
    }};
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => {{
        println!("{} {}", format!("{}[-]{}", $crate::colors::RED, $crate::colors::RESET), format!($($arg)+));
    }};
}

pub use command;
pub use error;
pub use plus;
