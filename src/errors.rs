use std::io;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to set sid: {0}")]
    SetSid(nix::errno::Errno),
    #[error("failed to spawn command: {0}")]
    SpawnCommand(io::Error),
    #[error("failed to wait for a command: {0}")]
    CommandWait(io::Error),
    #[error("failed to mount /proc: {0}")]
    MountProc(nix::errno::Errno),
    #[error("failed to mount /dev: {0}")]
    MountDev(nix::errno::Errno),
    #[error("failed to mount /sys: {0}")]
    MountSys(nix::errno::Errno),
    #[error("failed to parse fstab: {0}")]
    ParseFsTab(io::Error)
}

pub type Result<T> = std::result::Result<T, Error>;
