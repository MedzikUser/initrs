#[macro_export]
macro_rules! plus {
    ($($arg:tt)+) => {{
        use colored::Colorize;
        println!("{} {}", "[+]".green(), format!($($arg)+));
    }};
}

pub use plus;
