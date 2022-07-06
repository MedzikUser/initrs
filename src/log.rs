#[macro_export]
macro_rules! plus {
    ($($arg:tt)+) => {{
        use colored::Colorize;
        println!("{} {}", "[+]".green(), format!($($arg)+));
    }};
}

#[macro_export]
macro_rules! plus_yellow {
    ($($arg:tt)+) => {{
        use colored::Colorize;
        println!("{} {}", "[+]".yellow(), format!($($arg)+));
    }};
}

pub use plus;
pub use plus_yellow;
