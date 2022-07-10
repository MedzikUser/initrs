pub mod colors;
pub mod errors;
pub mod init;
pub mod initcli;
pub mod log;
pub mod unwrap;

pub const INIT_DIR: &str = "/etc/initrs";
pub const SERVICES_DIR: &str = "/etc/initrs/services";
pub const ENABLED_SERVICES_DIR: &str = "/etc/initrs/services/enabled";
