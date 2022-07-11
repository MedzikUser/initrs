use nix::unistd::Uid;

use crate::errors::{Error, Result};

/// Check if it executes as root user, if not return [Error::AccessDenied]
pub fn check_run_user() -> Result<()> {
    if !Uid::effective().is_root() {
        Err(Error::AccessDenied)?
    }

    Ok(())
}
