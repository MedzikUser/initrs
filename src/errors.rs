use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
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
}

pub type Result<T> = std::result::Result<T, Error>;
