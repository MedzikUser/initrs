use std::process::Command;

use crate::errors::{Error, Result};

/// Spawn and wait for command execution
pub fn run(line: &str) -> Result<()> {
    let status = Command::new("sh")
        .args(vec!["-c", line])
        .status()
        .map_err(Error::RunCommand)?;

    if !status.success() {
        Err(Error::ExitCommand(status.code().unwrap_or(-1)))?
    }

    Ok(())
}
