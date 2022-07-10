use core::fmt;
use std::io;

use thiserror::Error;

#[derive(Error)]
pub enum Error {
    #[error("failed to set sid: {0}")]
    SetSid(nix::errno::Errno),
    #[error("failed to run command: {0}")]
    RunCommand(io::Error),
    #[error("command exits with non-successful code: {0}")]
    ExitCommand(i32),
    #[error("failed to parse fstab: {0}")]
    ParseFsTab(io::Error),
    #[error("failed to read services directory: {0}")]
    ReadServicesDir(io::Error),
    #[error("failed to read service file content to string: {0}")]
    OpenServiceFile(io::Error),
    #[error("failed to parse service config: {0}")]
    ParseServiceFile(toml::de::Error),
    // Below this line is Error for Cli
    #[error("Service `{0}` not exists")]
    ServiceNotFound(String),
    #[error("Service `{0}` is already enabled")]
    ServiceAlreadyEnabled(String),
    #[error("Access denied")]
    AccessDenied,
    #[error("Failed to create symlink: {0}")]
    CreateSymlink(io::Error),
    #[error("Failed to delete file: {0}")]
    DeleteFile(io::Error),
    #[error("Failed to generate manpage: {0}")]
    GenerateManpage(io::Error),
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple(&self.to_string()).finish()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
