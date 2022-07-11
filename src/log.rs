#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{
        println!("{} {}", format!("{}::{}", $crate::colors::GREEN, $crate::colors::RESET), format!($($arg)+));
    }};
}

#[macro_export]
macro_rules! success {
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

pub use error;
pub use info;
pub use success;
